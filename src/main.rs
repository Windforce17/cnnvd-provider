use futures::StreamExt;
mod cnnvd;
mod cnnvdhandlers;
mod db;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;
use std::process::{exit, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::OnceCell;
use tracing::{info, Instrument};

pub static cnnvd_http_client: OnceCell<reqwest::Client> = OnceCell::const_new();
pub static DB: OnceCell<sqlx::PgPool> = OnceCell::const_new();

#[tokio::main]

async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();
    cnnvd::init_cnnvd_http_client().await;
    DB.set(
        sqlx::PgPool::connect("postgres://postgres:alanniubi666@localhost:5432/cnnvd")
            .await
            .unwrap(),
    )
    .unwrap();
    // let l = cnnvd::get_one_page(1, 5).await.unwrap();
    // l.iter().for_each(|x| {
    //     info!(
    //         "id: {}, cnnvd_code: {}, vul_type: {}",
    //         x.id, x.cnnvd_code, x.vul_type
    //     );
    // });
    cnnvd::sync_db_init().await;
    exit(0);
    //{"pageIndex":1,"pageSize":10,"keyword":"","hazardLevel":"","vulType":"","vendor":"","product":"","dateType":""}
    let max_count = cnnvd::get_max_count().await.unwrap();
    info!("max_count: {}", max_count);
}
