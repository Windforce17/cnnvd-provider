use crate::db::{CnnvdCollect, CnnvdProviderToken, CnnvdProviderUpdates};
use crate::DB;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ErrorRsp {
    msg: String,
}

pub struct CnnvdService;
impl CnnvdService {
    pub fn router() -> Router {
        let router = Router::new()
            .push(Router::with_path("/get_update_cnnvd").post(get_update_cnnvd))
            .push(Router::with_path("/confirm_update_cnnvd").post(confirm_update_Cnnvd))
            .push(Router::with_path("/fetch_cnnvd").post(fetch_cnnvd))
            .push(Router::with_path("/fetch_cnnvd_by_ids").post(fetch_Cnnvd_by_ids));

        router
    }
}

#[async_trait]
impl Writer for ErrorRsp {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.set_status_code(StatusCode::OK);
        res.render(Json(&self));
    }
}

#[derive(Deserialize)]
struct GetUpdateCnnvdReq {
    token: String,
    max_size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetUpdateCnnvdRsp {
    cnnvd_ids: Vec<i64>,
}

#[handler]
async fn get_update_cnnvd(req: &mut Request, rsp: &mut Response) -> Result<(), ErrorRsp> {
    let r: GetUpdateCnnvdReq = req.parse_json().await.map_err(|x| {
        error!("parse json failed: {}", x);
        let ersp = ErrorRsp {
            msg: "parse json failed".to_string(),
        };
        ersp
    })?;
    let token = r.token;
    let max_size = r.max_size;
    let db_pool = DB.get().unwrap();
    let ids = CnnvdProviderUpdates::get_update_cnnvd_id_by_token(&token, &db_pool)
        .await
        .map_err(|x| {
            error!("get update Cnnvd id failed: {:?}", x.source());

            let ersp = ErrorRsp {
                msg: "get update Cnnvd id failed".to_string(),
            };
            ersp
        })?;
    let r = GetUpdateCnnvdRsp { cnnvd_ids: ids };
    rsp.render(Json(r));
    Ok(())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ConfirmUpdateCnnvdReq {
    token: String,
    cnnvd_provider_ids: Vec<u64>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]

struct ConfirmUpdateCnnvdRsp {
    cnnvd_provider_id: Vec<u64>,
}

#[handler]
async fn confirm_update_Cnnvd(req: &mut Request, rsp: &mut Response) -> Result<(), ErrorRsp> {
    let r = req
        .parse_json::<ConfirmUpdateCnnvdReq>()
        .await
        .map_err(|x| {
            error!("parse json failed: {}", x);
            let ersp = ErrorRsp {
                msg: "parse json failed".to_string(),
            };
            ersp
        })?;
    let token = r.token;
    let Cnnvd_provider_ids = r.cnnvd_provider_ids;
    let db_pool = DB.get().unwrap();
    CnnvdProviderUpdates::delete_confirmed_Cnnvd_id_by_token(&token, Cnnvd_provider_ids, &db_pool)
        .await
        .map_err(|x| {
            error!("confirm update Cnnvd failed: {}", x);
            let ersp = ErrorRsp {
                msg: "confirm update Cnnvd failed".to_string(),
            };
            ersp
        })?;
    rsp.render(Json(ErrorRsp {
        msg: "".to_string(),
    }));
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CnnvdServiceDetail {
    cnnvd_provider_id: u64,
    cnnvd_source_json: String,
}

type GetUpadteCnnvdRsp = Vec<CnnvdServiceDetail>;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FetchCnnvdReq {
    max_count: u64,
    start_cnnvd_provider_id: u64,
}

#[handler]
async fn fetch_cnnvd(req: &mut Request, rsp: &mut Response) -> Result<(), ErrorRsp> {
    let r = req.parse_json::<FetchCnnvdReq>().await.map_err(|x| {
        error!("parse json failed: {}", x);
        let ersp = ErrorRsp {
            msg: "parse json failed".to_string(),
        };
        ersp
    })?;
    let max_count = r.max_count;
    let start_Cnnvd_provider_id = r.start_cnnvd_provider_id;
    let db_pool = DB.get().unwrap();
    let Cnnvds = CnnvdCollect::get_mmmmany_Cnnvd(start_Cnnvd_provider_id, max_count, db_pool)
        .await
        .map_err(|x| {
            error!("get Cnnvd failed: {}", x);
            let ersp = ErrorRsp {
                msg: "get Cnnvd failed".to_string(),
            };
            ersp
        })?;
    let r = Cnnvds
        .iter()
        .map(|x| CnnvdServiceDetail {
            cnnvd_provider_id: x.id as u64,
            cnnvd_source_json: x.cnnvd_source_json.clone(),
        })
        .collect::<GetUpadteCnnvdRsp>();
    rsp.render(Json(r));

    Ok(())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FetchCnnvdByIdsReq {
    Cnnvd_provider_ids: Vec<u64>,
}

#[handler]
async fn fetch_Cnnvd_by_ids(req: &mut Request, rsp: &mut Response) -> Result<(), ErrorRsp> {
    let r = req.parse_json::<FetchCnnvdByIdsReq>().await.map_err(|x| {
        error!("parse json failed: {}", x);
        let ersp = ErrorRsp {
            msg: "parse json failed".to_string(),
        };
        ersp
    })?;
    let Cnnvd_provider_ids = r.Cnnvd_provider_ids;
    let db_pool = DB.get().unwrap();
    let r = CnnvdCollect::get_by_ids(Cnnvd_provider_ids, db_pool)
        .await
        .map_err(|x| {
            error!("get Cnnvd failed: {}", x);
            let ersp = ErrorRsp {
                msg: "get Cnnvd failed".to_string(),
            };
            ersp
        })?;
    let r = r
        .iter()
        .map(|x| CnnvdServiceDetail {
            cnnvd_provider_id: x.id as u64,
            cnnvd_source_json: x.cnnvd_source_json.clone(),
        })
        .collect::<GetUpadteCnnvdRsp>();
    rsp.render(Json(r));
    Ok(())
}
