use axum::Router;
use axum::routing::get;

use crate::handlers::sys_role_handler;

pub fn sys_role_router() -> Router {
    Router::new().route("/sysroles", get(sys_role_handler::find_all))
        .route("/sysroles/:id", get(sys_role_handler::find))
}