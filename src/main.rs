use std::net::SocketAddr;

use axum::Extension;
use sqlx::{MySql, Pool};

mod routers;
mod domain;
mod dto;
mod handlers;
mod repository;
mod db;

#[derive(Clone)]
pub struct AppDbStat {
    pool: Pool<MySql>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    // let service_builder = ServiceBuilder::new();
    let pool = db::mysql_pool().await?;
    let app_stat = AppDbStat { pool };
    let routes = routers::routers()
        // 使用请求扩展进行外部传递
        .layer(Extension(app_stat))
        // 使用状态提取器进行外部传递
        // .with_state(app_stat)
        ;
    // service_builder.
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await?;
    Ok(())
}

