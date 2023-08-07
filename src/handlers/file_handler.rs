use axum::{Extension, Json};

use crate::AppDbStat;
use crate::domain::file::File;
use crate::repository::file_repository;

pub async fn find_all(Extension(app_stat): Extension<AppDbStat>) -> Json<Vec<File>> {
    let files: Vec<File> = file_repository::find_all(app_stat).await.unwrap();
    Json(files)
}