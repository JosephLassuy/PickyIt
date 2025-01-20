use std::time::SystemTime;

use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use chrono::{DateTime, Utc};
use sqlx::{prelude::FromRow, query, query_as};
use uuid::Uuid;

use time::OffsetDateTime;

use super::app_state::AppState;

use time::Duration as TimeDuration;

#[derive(Debug, Clone, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub account_id: Uuid,
    pub token: String,
    pub creation_date: DateTime<Utc>,
    pub expire_date: DateTime<Utc>,
    pub device: String,
    pub email: String,
}

impl Session {
    pub async fn create<S: AsRef<str>>(
        state: &AppState,
        account_id: Uuid,
        device: String,
        email: &S,
        expire_date: SystemTime,
        jar: CookieJar,
    ) -> Result<CookieJar, sqlx::Error> {
        let token = Uuid::new_v4().to_string();
        match query!(
            "INSERT INTO sessions (account_id, token,  expire_date, device, email) VALUES ($1, $2, $3, $4, $5)",
            account_id,
            token,
            DateTime::<Utc>::from(expire_date),
            device,
            email.as_ref()
        )
            .execute(&state.db)
            .await
        {
            Ok(_) => {
                let cookie_domain = &state.cookie_domain;

                let token_cookie = Cookie::build(("token", token))
                    .domain(cookie_domain.clone())
                    .expires(OffsetDateTime::from(expire_date))
                    .same_site(SameSite::Lax);
                let email_cookie = Cookie::build(("email", email.as_ref().to_string()))
                    .domain(cookie_domain.clone())
                    .expires(OffsetDateTime::from(expire_date))
                    .same_site(SameSite::Lax);
                let jar = jar
                    .add(token_cookie)
                    .add(email_cookie);
                println!("Session: {:?}", jar);
                Ok(jar)
            },
            Err(e) => Err(e),
        }
    }

    pub async fn create_and_get<S: AsRef<str>>(
        state: &AppState,
        account_id: Uuid,
        device: String,
        email: S,
        expire_date: SystemTime,
        jar: CookieJar,
    ) -> Result<(Session, CookieJar), sqlx::Error> {
        let token = Uuid::new_v4().to_string();
        let session = match query_as!(
            Session,
            "INSERT INTO sessions (account_id, token,  expire_date, device, email) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            account_id,
            token,
            DateTime::<Utc>::from(expire_date),
            device,
            email.as_ref()
        )
            .fetch_one(&state.db)
            .await
        {
            Ok(session) => session,
            Err(e) => return Err(e),
        };

        let mut session_cookie = Cookie::new("token", token);
        session_cookie.set_domain(state.cookie_domain.to_string());
        session_cookie.set_path("/");
        session_cookie.set_secure(true);
        session_cookie.set_http_only(true);
        session_cookie.set_same_site(SameSite::Lax);
        session_cookie.set_max_age(TimeDuration::seconds(
            expire_date
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        ));

        let mut email_cookie = Cookie::new("email", email.as_ref().to_string());
        email_cookie.set_domain(state.cookie_domain.to_string());
        email_cookie.set_path("/");
        email_cookie.set_secure(true);
        email_cookie.set_http_only(true);
        email_cookie.set_same_site(SameSite::Lax);
        email_cookie.set_max_age(TimeDuration::seconds(
            expire_date
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        ));
        let jar = jar.add(session_cookie).add(email_cookie);

        println!("Session2222: {:?}", jar);

        Ok((session, jar))
    }
}
