use anyhow::Context;
use anyhow::Result;
use serde_json::json;
pub async fn get_max_count() -> Result<u64> {
    let c = reqwest::Client::new();
    let rsp = c
        .post(crate::cnnvd_url)
        .json(&json!({
            "pageIndex": 1,
            "pageSize": 10,
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

pub async fn get_last_count() {}
