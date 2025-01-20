use axum::{extract::State, http::StatusCode};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use backend::util::app_state::AppState;
use sqlx::query;

pub async fn logout(State(state): State<AppState>, jar: CookieJar) -> (CookieJar, StatusCode) {
    let mut cookie_jar = jar;

    let token = cookie_jar.get("token").unwrap().value().to_string();
    let email = cookie_jar.get("email").unwrap().value().to_string();

    let remove_cookie_query = query!(
        "DELETE FROM sessions WHERE token = $1 OR email = $2",
        token,
        email
    )
    .execute(&state.db)
    .await;

    if remove_cookie_query.is_err() {
        return (cookie_jar, StatusCode::INTERNAL_SERVER_ERROR);
    }

    let mut token_cookie = Cookie::new("token", "");
    token_cookie.set_domain(state.cookie_domain.clone());
    token_cookie.set_path("/");
    token_cookie.set_max_age(time::Duration::seconds(-1));

    let mut email_cookie = Cookie::new("email", "");
    email_cookie.set_domain(state.cookie_domain);
    email_cookie.set_path("/");
    email_cookie.set_max_age(time::Duration::seconds(-1));

    cookie_jar = cookie_jar.add(token_cookie).add(email_cookie);

    (cookie_jar, StatusCode::OK)
}
