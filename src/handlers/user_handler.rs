use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;

use crate::AppDbStat;
use crate::domain::user::User;
use crate::dto::user_dto::CreateUser;
use crate::repository::user_repository;

// 这里有一个坑，请求和状态提取器必须放在参数的开头,因为最后一个参数必须是实现了FromRequest特性;
// 请求正文是只能使用一次的异步流，因此参数中只能有一个使用请求正文的提取程序。axum通过要求此类提取器是最后一个参数来强制执行这一点
// 如果您的提取程序不需要访问请求正文则实现FromRequestParts
// 请注意，无法同时实现 FromRequest 和 FromRequestParts，通过同时实现和直接针对同一类型，将使提取器不可用，除非它是包装另一个提取器
pub async fn create_user(Extension(app_stat): Extension<AppDbStat>, Json(payload): Json<CreateUser>)
                         -> (StatusCode, Json<User>) {
    let mut user = User::default();
    user.set_name(&payload.name.unwrap_or_default());
    user.set_age(payload.age.unwrap_or_default());
    user.set_gender(&payload.gender.unwrap_or_default());
    user.set_province(&payload.province.unwrap_or_default());
    user.set_city(&payload.city.unwrap_or_default());
    user.set_address(&payload.address.unwrap_or_default());
    user.set_phone(&payload.phone.unwrap_or_default());

    let result = match user_repository::create(&user, app_stat).await.unwrap() {
        true => (StatusCode::CREATED, Json(user)),
        false => (StatusCode::BAD_REQUEST, Json(user))
    };
    result
}

/* 使用请求扩展提取器进行外部传递 */

pub async fn find_all(Extension(app_stat): Extension<AppDbStat>) -> Json<Vec<User>> {
    let users = user_repository::find_all(app_stat).await.unwrap();
    Json(users)
}

pub async fn find(Extension(app_stat): Extension<AppDbStat>, Path(id): Path<i64>)
                  -> Json<User> {
    let user = user_repository::find(app_stat, id).await.unwrap();
    Json(user)
}

pub async fn update_by_id(Extension(app_stat): Extension<AppDbStat>, Json(user): Json<User>)
                          -> (StatusCode, String) {
    let result = match user_repository::update_by_id(&user, app_stat).await.unwrap() {
        true => (StatusCode::OK, "true".to_string()),
        false => (StatusCode::BAD_REQUEST, "false".to_string())
    };
    result
}

pub async fn delete(Extension(app_stat): Extension<AppDbStat>)
                    -> (StatusCode, String) {
    let result = match user_repository::delete(app_stat).await.unwrap() {
        true => (StatusCode::OK, "true".to_string()),
        false => (StatusCode::BAD_REQUEST, "false".to_string())
    };
    result
}

pub async fn delete_by_id(Extension(app_stat): Extension<AppDbStat>, Path(id): Path<i64>)
                          -> (StatusCode, String) {
    let result = match user_repository::delete_by_id(app_stat, id).await.unwrap() {
        true => (StatusCode::OK, "true".to_string()),
        false => (StatusCode::BAD_REQUEST, "false".to_string())
    };
    result
}
// 使用状态提取器进行外部传递
// pub async fn find_all(State(app_stat): State<AppStat>) -> Json<Vec<User>> {
//     let users = user_repository::find_all(app_stat).await;
//     Json(users)
// }
//
// pub async fn find(State(app_stat): State<AppStat>, Path(id): Path<i64>) -> Json<User> {
//     let user = user_repository::find(app_stat, id).await;
//     Json(user)
// }