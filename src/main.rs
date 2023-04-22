use futures::StreamExt;
mod cnnvd;
mod db;
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::{info, Instrument};

pub const cnnvd_url: &str = "https://www.cnnvd.org.cn/web/homePage/cnnvdVulList";

pub const DB: tokio::sync::OnceCell<sqlx::PgPool> = tokio::sync::OnceCell::new();
#[tokio::main]

async fn main() {
    tracing_subscriber::fmt::init();
    DB.set(
        sqlx::PgPool::connect("postgres://postgres:alanniubi666@localhost:5432/postgres")
            .await
            .unwrap(),
    )
    .unwrap();

    //{"pageIndex":1,"pageSize":10,"keyword":"","hazardLevel":"","vulType":"","vendor":"","product":"","dateType":""}
    let max_count = cnnvd::get_max_count().await.unwrap();
    info!("max_count: {}", max_count);
}
