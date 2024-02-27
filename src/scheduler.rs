use tokio_cron_scheduler::Job;
use tracing::info;

use crate::db::get_data;
use crate::email::{send_email, Email, Identity};

// 30 * 12-18 Mar *
// 30 minutes on everyday from 12 to 18 Mar

pub fn job() -> Job {
    Job::new_async_tz("1/30 * * * * *", chrono_tz::Asia::Kolkata, |_uuid, _l| {
        Box::pin(async move {
            info!("get data");
            let email = get_data().await.into_iter();
            for e in email {
                let p = Email {
                    from: Identity {
                        name: "Hermes".to_string(),
                        email: "hermes@localhost".to_string(),
                    },
                    to: Identity {
                        name: e.name,
                        email: e.email,
                    },
                    subject: "Hello".to_string(),
                    body: "Hello".to_string(),
                };
                // send_email(p).await;
                info!("send mail");
            }
        })
    })
    .unwrap()
}