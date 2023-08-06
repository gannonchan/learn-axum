use axum::body::Body;
use axum::Router;
use axum::routing::{get, post};

use crate::handlers::payment_handler;

pub fn payment_router() -> Router<(), Body> {
    Router::new().route("/payment", get(payment_handler::find_all))
        .route("/payment", post(payment_handler::create_payment))
}