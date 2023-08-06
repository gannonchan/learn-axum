use axum::{Extension, Json};
use axum::extract::Path;

use crate::AppDbStat;
use crate::domain::sys_role::SysRole;
use crate::repository::sys_role_repository;

pub async fn find_all(Extension(app_stat): Extension<AppDbStat>) -> Json<Vec<SysRole>> {
    let sys_roles = sys_role_repository::find_all(app_stat).await.unwrap();
    Json(sys_roles)
}

pub async fn find(Extension(app_stat): Extension<AppDbStat>, Path(id): Path<i64>) -> Json<SysRole> {
    let sys_role = sys_role_repository::find(app_stat, id).await.unwrap();
    Json(sys_role)
}