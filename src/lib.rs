pub mod routes;
pub mod shutdown;

use lettre::{transport::smtp::AsyncSmtpTransport, Tokio1Executor};
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub mailer: AsyncSmtpTransport<Tokio1Executor>,
}
