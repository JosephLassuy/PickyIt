use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, Extension, Json};
use backend::util::{app_state::AppState, user::User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CalendarItemCreationInformation {
    meal_item_id: Uuid,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    shared_with: Vec<Uuid>,
}

pub async fn create_calendar_item(
    Extension((user, _token)): Extension<(User, String)>,
    State(state): State<AppState>,
    Json(calendar_item): Json<CalendarItemCreationInformation>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!(
        "INSERT INTO calendar_items (user_id, shared_with, meal_item_id, start_date, end_date) VALUES ($1, $2, $3, $4, $5)",
        user.id,
        &calendar_item.shared_with,
        calendar_item.meal_item_id,
        calendar_item.start_date,
        calendar_item.end_date
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create calendar item: {}", e)))?;
    Ok(StatusCode::OK)
}

#[derive(Serialize)]
pub struct CalendarItemReturn {
    pub id: Uuid,
    pub meal_item_id: Option<Uuid>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

pub async fn get_calendar_items(
    Extension((user, _token)): Extension<(User, String)>,
    State(state): State<AppState>,
) -> Result<Json<Vec<CalendarItemReturn>>, (StatusCode, String)> {
    let calendar_items = sqlx::query_as!(
        CalendarItemReturn,
        "SELECT id, meal_item_id, start_date, end_date FROM calendar_items WHERE user_id = $1 OR $1 = ANY(shared_with)",
        user.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get calendar items: {}", e)))?;
    Ok(Json(calendar_items))
}

pub async fn delete_calendar_item(
    Extension((user, _token)): Extension<(User, String)>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!(
        "DELETE FROM calendar_items WHERE id = $1 AND user_id = $2",
        id,
        user.id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete calendar item: {}", e),
        )
    })?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct CalendarItemUpdateInformation {
    meal_item_id: Option<Uuid>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
}

pub async fn update_calendar_item(
    Extension((user, _token)): Extension<(User, String)>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(calendar_item): Json<CalendarItemUpdateInformation>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!(
        "UPDATE calendar_items SET meal_item_id = $1, start_date = $2, end_date = $3 WHERE id = $4 AND user_id = $5",
        calendar_item.meal_item_id,
        calendar_item.start_date,
        calendar_item.end_date,
        id,
        user.id
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update calendar item: {}", e)))?;
    Ok(StatusCode::OK)
}
