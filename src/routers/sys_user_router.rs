use axum::Router;
use axum::routing::get;

use crate::handlers::sys_user_handler;

pub fn sys_user_router() -> Router {
    Router::new().route("/sysusers", get(sys_user_handler::find_all))
        .route("/sysusers/:id", get(sys_user_handler::find))
}