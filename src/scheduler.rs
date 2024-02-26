use std::time::Duration;

use sqlx::{Pool, Postgres};
use tokio_cron_scheduler::Job;
use tracing::info;

use crate::email::{send_email, Email, Identity};

pub fn job() -> Job {
    Job::new_async("1/10 * * * * *", |_uuid, _l| {
        Box::pin(async move {
            let p = Email {
                from: Identity {
                    name: "Hermes".to_string(),
                    email: "hermes@localhost".to_string(),
                },
                to: Identity {
                    name: "Hermes".to_string(),
                    email: "hermes@localhost".to_string(),
                },
                subject: "Hello".to_string(),
                body: "Hello".to_string(),
            };
            send_email(p).await;
            info!("send mail");
        })
    })
    .unwrap()
}
