#![allow(dead_code)]
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;
mod cnnvd;
mod cnnvdhandlers;
mod db;
use salvo::prelude::*;
use tokio::sync::OnceCell;
use tracing::{error, info};

pub static CNNVD_HTTP_CLIENT: OnceCell<reqwest::Client> = OnceCell::const_new();
pub static DB: OnceCell<sqlx::PgPool> = OnceCell::const_new();

async fn init() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    //postgres://postgres:alanniubi666@localhost:5432/cnnvd
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let db_client = PgPoolOptions::new()
        .max_connections(300)
        .connect(&database_url)
        .await
        .unwrap();

    DB.set(db_client).unwrap();
    info!("init db pool success!");
    cnnvd::init_cnnvd_http_client().await;
}

#[tokio::main]
async fn main() {
    init().await;
    let need_init = std::env::var("INIT").unwrap_or_default();
    if need_init != "".to_string() {
        info!("init cnnvd db,please wait...");
        cnnvd::sync_db_init().await;
    }
    info!("start update timer!");

    tokio::spawn(async move {
        loop {
            //update every half hour
            tokio::time::sleep(tokio::time::Duration::from_secs(600)).await;
            info!("update cnnvd,please wait...");
            let _ = cnnvd::start_sync().await.map_err(|e| {
                error!("update cnnvd error:{:?}", e.source());
            });
        }
    });
    let router = cnnvdhandlers::CnnvdService::router();
    let at = TcpListener::new("0.0.0.0:5801").bind().await;
    Server::new(at).serve(router).await;
}
