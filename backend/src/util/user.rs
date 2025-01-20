use chrono::{DateTime, Utc};

use sqlx::{prelude::FromRow, query_as};
use uuid::Uuid;

use super::{app_state::AppState, session::Session};

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub email_verified: Option<DateTime<Utc>>,
    pub creation_date: DateTime<Utc>,
}

pub fn validate_password(password: &str) -> bool {
    let password = password.trim();
    password.len() >= 8
        && password.chars().any(|c| c.is_digit(10))
        && password.chars().any(|c| c.is_uppercase())
}

impl User {
    pub async fn create<S: AsRef<str>>(
        state: &AppState,
        first_name: String,
        last_name: String,
        email: S,
    ) -> Result<User, sqlx::Error> {
        match sqlx::query_as!(
            User,
            "INSERT INTO users (first_name, last_name, email) VALUES ($1, $2, $3) RETURNING *",
            first_name,
            last_name,
            email.as_ref().to_lowercase()
        )
        .fetch_one(&state.db)
        .await
        {
            Ok(user) => Ok(user),
            Err(error) => Err(error),
        }
    }

    pub async fn get_by_email<S: AsRef<str>>(state: &AppState, email: S) -> Result<User, String> {
        let email = email.as_ref().to_lowercase();
        let user = query_as!(User, "SELECT * FROM users where email = $1", email)
            .fetch_optional(&state.db)
            .await;
        match user {
            Ok(user) => {
                if user.is_none() {
                    Err("User not found".to_string())
                } else {
                    Ok(user.unwrap())
                }
            }
            Err(_) => {
                eprintln!("Error: {}", user.unwrap_err());
                Err("User not found".to_string())
            }
        }
    }

    pub async fn is_logged_in(&self, state: &AppState, token: String) -> bool {
        let email = &self.email;

        let keys: Option<Session> =
            query_as!(Session, "SELECT * FROM sessions where token = $1", token)
                .fetch_optional(&state.db)
                .await
                .unwrap();

        if keys.is_none() {
            return false;
        }
        let session = keys.unwrap();
        if &session.email == email {
            return true;
        }
        return false;
    }
}
