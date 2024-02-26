pub mod email;
pub mod scheduler;

use serde::Serialize;

#[derive(Serialize)]
pub struct PingResponse {
    pub ok: bool,
    pub msg: String,
}
