use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::CookieJar;
use serde::Serialize;

use backend::util::app_state::AppState;
use backend::util::user::User;
#[derive(Serialize)]
pub struct VerifyResponse {
    authenticated: bool,
}

pub async fn verify_auth(
    State(state): State<AppState>,
    cookie: CookieJar,
) -> Result<Json<VerifyResponse>, StatusCode> {
    let token = cookie.get("token").map(|c| c.value().to_string());
    let email = cookie.get("email").map(|c| c.value().to_string());

    match (token, email) {
        (Some(token), Some(email)) => {
            let user = match User::get_by_email(&state, email.clone()).await {
                Ok(user) => user,
                Err(_) => {
                    return Ok(Json(VerifyResponse {
                        authenticated: false,
                    }))
                }
            };

            if !user.is_logged_in(&state, token.clone()).await {
                return Ok(Json(VerifyResponse {
                    authenticated: false,
                }));
            }

            Ok(Json(VerifyResponse {
                authenticated: true,
            }))
        }
        _ => Ok(Json(VerifyResponse {
            authenticated: false,
        })),
    }
}
