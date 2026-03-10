use secrecy::ExposeSecret;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::configuration::Settings;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub async fn new(configuration: &Settings) -> AppState {
        match PgPoolOptions::new()
            .max_connections(configuration.database.max_connections)
            .connect(configuration.database.connection_string().expose_secret())
            .await
        {
            Ok(pool) => AppState { db: pool },
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        }
    }
}
