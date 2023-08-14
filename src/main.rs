use std::net::SocketAddr;
use std::time::Duration;

use axum::Extension;
use sqlx::{MySql, Pool};
use tower::ServiceBuilder;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tower_http::timeout::TimeoutLayer;

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
    let db_uri = dotenv::var("DATABASE_URI")?;
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    // let service_builder = ServiceBuilder::new();
    let pool = db::mysql_pool(db_uri).await?;
    let app_stat = AppDbStat { pool };
    let mut tera = tera::Tera::new("static1/templates/**/*")?;
    tera.full_reload()?;
    // 处理跨域访问
    let cors_layer = CorsLayer::new()
        .allow_headers(AllowHeaders::any())
        .allow_methods(AllowMethods::any())
        .allow_credentials(false)
        .allow_origin(AllowOrigin::any())
        .max_age(Duration::from_secs(3600));
    let middleware = ServiceBuilder::new()
        // 处理超时请求
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        // 添加跨域处理层
        .layer(cors_layer)
        ;
    let routes = routers::routers()
        // 使用请求扩展进行外部传递
        .layer(Extension(app_stat))
        .layer(Extension(tera))
        .layer(middleware)
        // 使用状态提取器进行外部传递
        // .with_state(app_stat)
        ;
    // service_builder.
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await?;
    Ok(())
}

