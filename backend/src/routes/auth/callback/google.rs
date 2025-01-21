use std::time::SystemTime;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use chrono::Duration;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use sqlx::query_as;
use time::Duration as TimeDuration;

use backend::util::{account::Account, app_state::AppState, session::Session, user::User};

pub fn create_google_oauth_client() -> BasicClient {
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID");
    let google_client_secret =
        std::env::var("GOOGLE_CLIENT_SECRET").expect("Missing GOOGLE_CLIENT_SECRET");
    let redirect_url = std::env::var("GOOGLE_REDIRECT_URL").expect("Missing GOOGLE_REDIRECT_URL");
    println!("GOOGLE_CLIENT_ID: {}", google_client_id);
    println!("GOOGLE_CLIENT_SECRET: {}", google_client_secret);
    println!("GOOGLE_REDIRECT_URL: {}", redirect_url);

    let client = BasicClient::new(
        ClientId::new(google_client_id),
        Some(ClientSecret::new(google_client_secret)),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

    client
}

pub async fn google_login() -> impl IntoResponse {
    let client = create_google_oauth_client();

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .url();

    Redirect::to(auth_url.as_str())
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    email: String,
    given_name: Option<String>,
    family_name: Option<String>,
    sub: Option<String>,
}

pub async fn google_callback(
    Query(query): Query<AuthRequest>,
    State(state): State<AppState>,
) -> Result<(CookieJar, Redirect), (axum::http::StatusCode, String)> {
    let client = create_google_oauth_client();

    let token = client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await
        .map_err(|e| {
            println!("Detailed error: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to exchange code: {}", e),
            )
        })?;

    let user_info: GoogleUserInfo = state
        .reqwest_client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get user info: {}", e),
            )
        })?
        .json()
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse user info: {}", e),
            )
        })?;

    let user = match User::get_by_email(&state, &user_info.email).await {
        Ok(user) => user,
        Err(_) => User::create(
            &state,
            user_info
                .given_name
                .unwrap_or_else(|| "Unknown".to_string()),
            user_info.family_name.unwrap_or_else(|| "".to_string()),
            &user_info.email,
        )
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create user: {}", e),
            )
        })?,
    };

    let account =
        match Account::get_by_user_id_and_provider(&state, user.id, "google".to_string()).await {
            Ok(account) => account,
            Err(_) => sqlx::query_as!(
            Account,
            "INSERT INTO accounts (user_id, provider, google_id) VALUES ($1, $2, $3) RETURNING *",
            user.id,
            "google",
            user_info.sub
        )
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to create account: {}", e),
                )
            })?,
        };

    let session_duration = Duration::days(30);
    let session = Session::create_and_get(
        &state,
        account.id,
        "Google OAuth".to_string(),
        &user_info.email,
        SystemTime::now() + std::time::Duration::from_secs(session_duration.num_seconds() as u64),
        CookieJar::new(),
    )
    .await
    .map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create session: {}", e),
        )
    })?;

    let accounts = query_as!(Account, "SELECT * FROM accounts")
        .fetch_all(&state.db)
        .await;
    println!("CallBack Google Accounts: {:?}", accounts);
    let users = query_as!(User, "SELECT * FROM users")
        .fetch_all(&state.db)
        .await;
    println!("CallBack Google Users: {:?}", users);

    let mut session_cookie = Cookie::new("token", session.0.token.to_string());
    session_cookie.set_domain(state.cookie_domain.to_string());
    session_cookie.set_path("/");
    session_cookie.set_secure(true);
    session_cookie.set_http_only(true);
    session_cookie.set_same_site(SameSite::Lax);
    session_cookie.set_max_age(TimeDuration::seconds(session_duration.num_seconds()));

    let mut email_cookie = Cookie::new("email", user_info.email);
    email_cookie.set_domain(state.cookie_domain.to_string());
    email_cookie.set_path("/");
    email_cookie.set_secure(true);
    email_cookie.set_http_only(true);
    email_cookie.set_same_site(SameSite::Lax);
    email_cookie.set_max_age(TimeDuration::seconds(session_duration.num_seconds()));

    Ok((session.1, Redirect::to(&state.domain)))
}
