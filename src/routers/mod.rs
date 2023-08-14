use axum::{Extension, Router};
use axum::body::Body;
use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use tera::{Context, Tera};

use payment_router::payment_router;
use user_router::user_router;

use crate::routers::file_router::file_router;
use crate::routers::sys_role_router::sys_role_router;
use crate::routers::sys_user_router::sys_user_router;

pub(crate) mod user_router;
pub(crate) mod payment_router;
pub(crate) mod sys_user_router;
pub(crate) mod sys_role_router;
pub(crate) mod file_router;

pub fn routers() -> Router<(), Body> {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/tera", get(tera))
        .route("/upload", post(upload))
        .merge(payment_router())
        .merge(user_router())
        .merge(sys_user_router())
        .merge(sys_role_router())
        .merge(file_router())
}

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn tera(Extension(tera): Extension<Tera>) -> Result<Html<String>, String> {
    let mut context = Context::new();
    context.insert("message", "testTera");
    let page = tera.render("index.html", &context)
        .map_err(|err| err.to_string())?;
    Ok(Html(page))
}

/// 上传操作
async fn upload(mut multipart: Multipart) -> Result<(StatusCode, String), String> {
    if let Some(file) = multipart.next_field().await.unwrap() {
        let filename = file.file_name().unwrap().to_string(); // 上传的文件名
        let data = file.bytes().await.unwrap(); // 上传的文件的内容

        // 保存上传的文件
        tokio::fs::write(&filename, &data)
            .await
            .map_err(|err| err.to_string())?;
        return Ok((StatusCode::CREATED, format!(
            "【上传的文件】文件名：{:?}, 文件大小：{}",
            filename,
            data.len()
        )));
    }
    Ok((StatusCode::NO_CONTENT, String::from("没有上传文件")))
}