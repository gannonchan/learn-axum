use axum::{Extension, Json};
use axum::extract::Path;

use crate::AppDbStat;
use crate::domain::sys_user::SysUser;
use crate::repository::sys_user_repository;

pub async fn find_all(Extension(app_stat): Extension<AppDbStat>) -> Json<Vec<SysUser>> {
    let sys_users = sys_user_repository::find_all1(app_stat).await.unwrap();
    Json(sys_users)
}

pub async fn find(Extension(app_stat): Extension<AppDbStat>, Path(id): Path<i64>) -> Json<SysUser> {
    let sys_user = sys_user_repository::find(app_stat, id).await.unwrap();
    Json(sys_user)
}