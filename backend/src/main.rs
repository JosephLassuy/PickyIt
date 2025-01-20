use std::env;

use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use backend::{middleware::auth::auth_middleware, util::app_state::AppState};
use routes::{
    auth::{
        callback::google::{google_callback, google_login},
        logout::logout,
        verify::verify_auth,
    },
    calendar::{
        create_calendar_item, delete_calendar_item, get_calendar_items, update_calendar_item,
    },
    library::{
        ingredient::{create_ingredient_item, delete_ingredient_item, get_ingredient_items},
        meal::{create_meal_item, delete_meal_item, get_meal_item, get_meal_items},
    },
};
use tower_http::cors::CorsLayer;

mod routes;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    dotenvy::dotenv().ok();

    let origins: Vec<_> = env::var("ALLOWED_ORIGINS")
        .expect("ALLOWED_ORIGINS must be set")
        .split(',')
        .map(|s| s.trim().parse().expect("Invalid origin URL"))
        .collect();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(origins)
        .allow_credentials(true)
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .expose_headers([CONTENT_TYPE, AUTHORIZATION]);

    let state = AppState::new().await;

    let auth_router = Router::new()
        .route("/login/google", get(google_login))
        .route("/login/google/callback", get(google_callback))
        .route("/verify", get(verify_auth))
        .route("/logout", get(logout));

    let food_router = Router::new()
        .route("/ingredient", get(get_ingredient_items))
        .route("/ingredient", post(create_ingredient_item))
        .route("/ingredient/{id}", delete(delete_ingredient_item))
        .route("/meal", get(get_meal_items))
        .route("/meal", post(create_meal_item))
        .route("/meal/{id}", get(get_meal_item))
        .route("/meal/{id}", delete(delete_meal_item))
        .route("/calendar", get(get_calendar_items))
        .route("/calendar", post(create_calendar_item))
        .route("/calendar/{id}", put(update_calendar_item))
        .route("/calendar/{id}", delete(delete_calendar_item))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let app = Router::new()
        .merge(auth_router)
        .merge(food_router)
        .with_state(state)
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
