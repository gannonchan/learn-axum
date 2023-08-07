use axum::body::Body;
use axum::Router;
use axum::routing::get;

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