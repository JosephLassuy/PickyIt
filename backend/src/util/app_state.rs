use std::env;

use reqwest::Client as ReqwestClient;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub reqwest_client: ReqwestClient,
    pub db: Pool<Postgres>,
    pub domain: String,
    pub cookie_domain: String,
}

impl AppState {
    pub async fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();

        Self {
            reqwest_client: ReqwestClient::new(),
            db: pool,
            domain: env::var("DOMAIN").expect("DOMAIN must be set"),
            cookie_domain: env::var("COOKIE_DOMAIN").expect("cookie_domain must be set"),
        }
    }
}
