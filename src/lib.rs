pub mod db;
pub mod email;
pub mod scheduler;
pub mod shutdown;

use serde::Serialize;

#[derive(Serialize)]
pub struct PingResponse {
    pub ok: bool,
    pub msg: String,
}
