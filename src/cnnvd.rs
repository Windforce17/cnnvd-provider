use crate::{
    db::{CnnvdCollect, CnnvdCollectUpdate},
    DB,
};
use anyhow::Context;
use anyhow::Result;
use reqwest::header::HeaderMap;
use serde_json::json;
use tokio::stream;
use tracing::{error, info};
pub async fn init_cnnvd_http_client() {
    let mut bypass_headers = HeaderMap::new();
    bypass_headers.append("Content-Type", "application/json".parse().unwrap());
    bypass_headers.append("Accept", "application/json".parse().unwrap());
    bypass_headers.append("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
    bypass_headers.append(
        "Accept-Language",
        "zh-CN,zh;q=0.9,en;q=0.8".parse().unwrap(),
    );
    bypass_headers.append("Connection", "keep-alive".parse().unwrap());
    bypass_headers.append("Host", "www.cnnvd.org.cn".parse().unwrap());
    bypass_headers.append("Origin", "https://www.cnnvd.org.cn".parse().unwrap());
    bypass_headers.append(
        "Referer",
        "https://www.cnnvd.org.cn/web/homePage.html"
            .parse()
            .unwrap(),
    );
    bypass_headers.append("Sec-Fetch-Dest", "empty".parse().unwrap());
    bypass_headers.append("Sec-Fetch-Mode", "cors".parse().unwrap());
    bypass_headers.append("Sec-Fetch-Site", "same-origin".parse().unwrap());
    bypass_headers.append(
        "User-Agent",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36"
            .parse()
            .unwrap(),
    );
    bypass_headers.append("X-Requested-With", "XMLHttpRequest".parse().unwrap());
    let client = reqwest::Client::builder()
        .default_headers(bypass_headers)
        .build()
        .unwrap();
    crate::cnnvd_http_client.set(client).unwrap();
}

pub async fn get_max_count() -> Result<u64> {
    let cnnvd_api: &str = "https://www.cnnvd.org.cn/web/homePage/cnnvdVulList";
    let rsp = crate::cnnvd_http_client
        .get()
        .unwrap()
        .post(cnnvd_api)
        .json(&json!({
            "pageIndex": 1,
            "pageSize": 1,
            "keyword": "",
            "hazardLevel": "",
            "vulType": "",
            "vendor": "",
            "product": "",
            "dateType": "",
        }))
        .send()
        .await
        .with_context(|| "Failed to send request")?
        .json::<serde_json::Value>()
        .await
        .with_context(|| "Failed to parse response")?;
    rsp.as_object()
        .with_context(|| "Failed to get object")?
        .get("data")
        .with_context(|| "Failed to get data")?
        .as_object()
        .with_context(|| "Failed to get object")?
        .get("total")
        .with_context(|| "Failed to get total")?
        .as_u64()
        .with_context(|| "Failed to get u64")
}

pub async fn get_last_count() -> Result<i64> {
    let last_counts = CnnvdCollectUpdate::get_last_counts(DB.get().unwrap())
        .await
        .with_context(|| "Failed to get last counts")?
        .unwrap_or_default();
    Ok(last_counts)
}

/*
{
    "code": 200,
    "success": true,
    "message": "操作成功",
    "data": {
        "total": 212240,
        "records": [
            {
                "id": "fdb876e6819744eda5e2a11775f3f8c1",
                "vulName": "INEA ME RTU 安全漏洞",
                "cnnvdCode": "CNNVD-202304-1727",
                "cveCode": "CVE-2023-2131",
                "hazardLevel": null,
                "createTime": "2023-04-21",
                "publishTime": "2023-04-20",
                "updateTime": "2023-04-21",
                "typeName": "其他",
                "vulType": "0"
            }
        ],
        "pageIndex": 1,
        "pageSize": 1
    },
    "time": "2023-04-23 17:42:27"
}
 */

//don't care about other fields
#[derive(serde::Deserialize)]
pub struct CnnvdRsp {
    pub data: CnnvdData,
}
#[derive(serde::Deserialize)]
pub struct CnnvdData {
    pub records: Vec<CnnvdRecord>,
    #[serde(rename = "pageIndex")]
    pub page_index: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
}
#[derive(serde::Deserialize)]
pub struct CnnvdRecord {
    pub id: String,
    #[serde(rename = "cnnvdCode")]
    pub cnnvd_code: String,
    #[serde(rename = "vulType")]
    pub vul_type: String,
}
pub async fn get_one_page(page_index: u64, page_size: u64) -> Result<Vec<CnnvdRecord>> {
    let cnnvd_api: &str = "https://www.cnnvd.org.cn/web/homePage/cnnvdVulList";
    if page_size > 50 {
        return Err(anyhow::anyhow!("pageSize must less than 50"));
    }
    let rsp = crate::cnnvd_http_client
        .get()
        .unwrap()
        .post(cnnvd_api)
        .json(&json!({
            "pageIndex": page_index,
            "pageSize": page_size,
        }))
        .send()
        .await
        .with_context(|| "Failed to send request")?
        .json::<CnnvdRsp>()
        .await
        .with_context(|| "Failed to parse response")?;
    if rsp.data.page_index != page_index {
        return Err(anyhow::anyhow!("pageIndex not match"));
    }
    if rsp.data.page_size != page_size {
        return Err(anyhow::anyhow!("pageSize not match"));
    }
    Ok(rsp.data.records)
}

pub async fn get_one_record_detail<'a>(
    id: &'a str,
    cnnvd_code: &'a str,
    vul_type: &'a str,
) -> Result<String> {
    let cnnvd_api: &str = "https://www.cnnvd.org.cn/web/cnnvdVul/getCnnnvdDetailOnDatasource";
    let rsp = crate::cnnvd_http_client
        .get()
        .unwrap()
        .post(cnnvd_api)
        .json(&json!({
            "cnnvdCode": cnnvd_code,
            "id":id,
            "vulType":vul_type
        }))
        .send()
        .await
        .with_context(|| "Failed to send request")?
        .json::<serde_json::Value>()
        .await
        .with_context(|| "Failed to parse response")?
        .as_object()
        .with_context(|| "Failed to get object")?
        .get("data")
        .with_context(|| "Failed to get data")?
        .as_object()
        .with_context(|| "Failed to get object")?
        .get("cnnvdDetail")
        .with_context(|| "Failed to get vulInfo")?
        .to_string();
    Ok(rsp)
}
// pub async fn
use futures::StreamExt;
use std::time::Duration;
pub async fn sync_db_init() {
    // sync all cnnvd meta data first; cnnvd will change page index!
    let max_count = get_max_count().await.unwrap();
    let page_size = 50;
    let mut total_page = max_count / page_size;
    info!("total_page:{}", total_page);
    total_page += 1;
    let mut tasks = futures::stream::FuturesUnordered::new();
    while total_page >= 1 {
        let mut o = get_one_page(total_page, page_size).await;

        while o.is_err() {
            error!("get_one_page error:{:?}", o.err().unwrap());
            error!(page_index = total_page);
            tokio::time::sleep(Duration::from_secs(1)).await;
            o = get_one_page(total_page, page_size).await;
        }
        let o = o.unwrap();
        let insert_db_task =
            futures::stream::iter(o.into_iter()).for_each_concurrent(50, |x| async move {
                let db_pool = DB.get().unwrap();
                //insert into db
                CnnvdCollect::upsert(&x.id, &x.cnnvd_code, &x.vul_type, "", db_pool)
                    .await
                    .unwrap();
            });
        // DB is veeeeerrrryyy fast!
        tasks.push(tokio::spawn(insert_db_task));

        total_page -= 1;
    }
    // wait all task done
    while let Some(_) = tasks.next().await {}
}
