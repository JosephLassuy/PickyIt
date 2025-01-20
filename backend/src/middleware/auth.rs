use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use serde::Serialize;

use crate::util::app_state::AppState;
use crate::util::user::User;

#[derive(Debug, Serialize)]
pub struct AuthMiddlewareResponse {
    pub verified_email: bool,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    cookie: CookieJar,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, (StatusCode, Json<AuthMiddlewareResponse>)> {
    let token = cookie.get("token").map(|c| c.value().to_string());
    let email = cookie.get("email").map(|c| c.value().to_string());

    let (user, token) = match (token, email) {
        (Some(token), Some(email)) => {
            let temp_user = User::get_by_email(&state, email.clone())
                .await
                .map_err(|_| {
                    eprintln!("User not found middleware");
                    StatusCode::UNAUTHORIZED
                })
                .unwrap();
            if !temp_user.is_logged_in(&state, token.clone()).await {
                eprintln!("User not logged in middleware");
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(AuthMiddlewareResponse {
                        verified_email: true,
                    }),
                ));
            }

            (temp_user.clone(), token.clone())
        }
        _ => {
            eprintln!("No token or email middleware");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(AuthMiddlewareResponse {
                    verified_email: false,
                }),
            ));
        }
    };

    request.extensions_mut().insert((user.clone(), token));
    Ok(next.run(request).await)
}
