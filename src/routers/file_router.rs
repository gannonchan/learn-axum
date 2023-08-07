use axum::Router;
use axum::routing::get;
use crate::handlers::file_handler;

pub fn file_router() ->Router{
    Router::new().route("/files",get(file_handler::find_all))
}