use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;
use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database: Database,
    pub smtp: Smtp,
    pub mode: String,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl Database {
    pub fn connection_string(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .host(&self.host)
            .database(&self.database_name)
            .ssl_mode(ssl_mode)
            .log_statements(tracing::log::LevelFilter::Trace)
    }
}

#[derive(Debug, Clone)]
pub struct Smtp {
    pub username: String,
    pub password: String,
    pub server: String,
}

pub fn get_config() -> Settings {
    if env::var("MODE").is_err() {
        env::set_var("MODE", "development");
    }

    if env::var("RUST_LOG").is_err() {
        env::set_var(
            "RUST_LOG",
            "hermes=debug,tower_http=debug,axum::rejection=trace,tokio=trace,runtime=trace",
        );
    }

    if env::var("MODE").unwrap() == "production" {
        let db_config = Database {
            username: env::var("DATABASE_USERNAME").unwrap(),
            password: env::var("DATABASE_PASSWORD").unwrap(),
            port: env::var("DATABASE_PORT").unwrap().parse().unwrap(),
            host: env::var("DATABASE_HOST").unwrap(),
            database_name: env::var("DATABASE_NAME").unwrap(),
            require_ssl: true,
        };
        let smtp_config = Smtp {
            username: env::var("SMTP_USERNAME").unwrap(),
            password: env::var("SMTP_PASSWORD").unwrap(),
            server: env::var("SMTP_SERVER").unwrap(),
        };
        Settings {
            database: db_config,
            smtp: smtp_config,
            mode: "production".to_string(),
        }
    } else {
        let db_config = Database {
            username: "postgres".to_string(),
            password: "password".to_string(),
            port: 5432,
            host: "localhost".to_string(),
            database_name: "techfiesta24".to_string(),
            require_ssl: false,
        };
        let smtp_config = Smtp {
            username: "local@host.com".to_string(),
            password: "".to_string(),
            server: "".to_string(),
        };
        Settings {
            database: db_config,
            smtp: smtp_config,
            mode: "development".to_string(),
        }
    }
}
