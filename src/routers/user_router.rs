use axum::body::Body;
use axum::Router;
use axum::routing::{delete, get, post, put};

use crate::handlers::user_handler::{self};

pub fn user_router() -> Router<(), Body> {
    Router::new()
        .route("/users", get(user_handler::find_all))
        .route("/users/:id", get(user_handler::find))
        // `POST /users` goes to `create_user`
        .route("/users", post(user_handler::create_user))
        // `PUT /users goes to update_by_id`
        .route("/users", put(user_handler::update_by_id))
        // `DELETE /users goes to delete`
        .route("/users", delete(user_handler::delete))
        // `DELETE /users/1 goes to delete`
        .route("/users/:id", delete(user_handler::delete_by_id))
}