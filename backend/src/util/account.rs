use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use sqlx::{prelude::*, query_as};
use uuid::Uuid;

use super::app_state::AppState;

#[derive(Debug, Clone, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub google_id: Option<String>,
    pub password: Option<String>,
    pub creation_date: DateTime<Utc>,
}

impl Account {
    pub async fn get_by_user_id(
        state: &AppState,
        user_id: Uuid,
    ) -> Result<Vec<Account>, sqlx::Error> {
        query_as!(
            Account,
            "SELECT * FROM accounts WHERE user_id = $1",
            user_id
        )
        .fetch_all(&state.db)
        .await
    }
    pub async fn create_email(
        state: &AppState,
        user_id: Uuid,
        password: String,
    ) -> Result<Account, sqlx::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        let account = sqlx::query_as!(
            Account,
            "INSERT INTO accounts (user_id, provider, password) VALUES ($1, $2, $3) RETURNING *",
            user_id,
            "email",
            password_hash
        )
        .fetch_one(&state.db)
        .await;
        if account.is_err() {
            return Err(account.err().unwrap());
        }
        Ok(account.unwrap())
    }

    pub async fn get_by_user_id_and_provider(
        state: &AppState,
        user_id: Uuid,
        provider: String,
    ) -> Result<Account, sqlx::Error> {
        query_as!(
            Account,
            "SELECT * FROM accounts WHERE user_id = $1 AND provider = $2",
            user_id,
            provider
        )
        .fetch_one(&state.db)
        .await
    }
}
