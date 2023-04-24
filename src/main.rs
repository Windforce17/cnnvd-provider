use cnnvd::sync_new_update;
use futures::StreamExt;
use salvo::prelude::TcpListener;
use tracing_subscriber::EnvFilter;
mod cnnvd;
mod cnnvdhandlers;
mod db;
use std::process::{exit, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::OnceCell;
use tracing::{error, info};

pub static cnnvd_http_client: OnceCell<reqwest::Client> = OnceCell::const_new();
pub static DB: OnceCell<sqlx::PgPool> = OnceCell::const_new();

#[tokio::main]

async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    cnnvd::init_cnnvd_http_client().await;
    DB.set(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(500)
            .connect("postgres://postgres:alanniubi666@localhost:5432/cnnvd")
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
    // cnnvd::sync_db_init().await;
    info!("sync_new_update");
    cnnvd::sync_new_update().await.unwrap();
    cnnvd::sync_empty_vuls().await.unwrap();

    exit(0);
    let router = cnnvdhandlers::CnnvdService::router();
    let at = TcpListener::new("0.0.0.0:5801").bind().await;
    Server::new(acceptor).serve(router).await;
}
