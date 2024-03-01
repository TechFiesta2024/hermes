mod batch_send_email;
mod health_check;
mod send_email;
mod verify_key;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::AppState;

pub fn routes(state: AppState) -> Router {
    let email_router = Router::new()
        .route("/send", post(send_email::send_email))
        .route("/batch_send", post(batch_send_email::batch_send_email))
        .layer(middleware::from_fn(verify_key::verify_key))
        .with_state(state);

    Router::new()
        .route("/health_check", get(health_check::health_check))
        .merge(email_router)
}
