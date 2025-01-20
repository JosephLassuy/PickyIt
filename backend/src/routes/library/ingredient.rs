use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, Extension, Json};
use backend::util::{app_state::AppState, user::User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct IngredientItemCreationInformation {
    name: String,
}

pub async fn create_ingredient_item(
    Extension((user, _token)): Extension<(User, String)>,
    State(state): State<AppState>,
    Json(ingredient_item): Json<IngredientItemCreationInformation>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!(
        "INSERT INTO ingredient_items (name, creator_id) VALUES ($1, $2)",
        ingredient_item.name,
        user.id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create ingredient item: {}", e),
        )
    })?;
    Ok(StatusCode::OK)
}

#[derive(Serialize)]
pub struct IngredientItem {
    pub id: Uuid,
    pub name: String,
    pub creator_id: Uuid,
}
#[derive(Serialize)]
pub struct IngredientItemsResponse {
    ingredient_items: Vec<IngredientItem>,
}
#[derive(Deserialize)]
pub struct GetIngredientItemsInformation {
    start: Option<i64>,
    end: Option<i64>,
    search: Option<String>,
    mine: Option<bool>,
}

pub async fn get_ingredient_items(
    Extension((user, _token)): Extension<(User, String)>,
    state: State<AppState>,
    Query(params): Query<GetIngredientItemsInformation>,
) -> Result<Json<IngredientItemsResponse>, (StatusCode, String)> {
    let start = params.start.unwrap_or(0);
    let end = params.end.unwrap_or(10);

    let query: Vec<IngredientItem> = if let Some(search) = params.search {
        if params.mine.unwrap_or(false) {
            sqlx::query_as!(
                IngredientItem,
                "SELECT id, name, creator_id FROM ingredient_items WHERE name ILIKE $1 AND creator_id = $2 ORDER BY name ASC OFFSET $3 LIMIT $4",
                format!("%{}%", search),
                user.id,
                start,
                end
            )
            .fetch_all(&state.db)
            .await
        } else {
            sqlx::query_as!(
                IngredientItem,
                "SELECT id, name, creator_id FROM ingredient_items WHERE name ILIKE $1 ORDER BY name ASC OFFSET $2 LIMIT $3",
                format!("%{}%", search),
                start,
                end
            )
            .fetch_all(&state.db)
            .await
        }
    } else {
        if params.mine.unwrap_or(false) {
            sqlx::query_as!(
                IngredientItem,
                "SELECT id, name, creator_id FROM ingredient_items WHERE creator_id = $1 ORDER BY name ASC OFFSET $2 LIMIT $3",
                user.id,
                start,
                end
            )
            .fetch_all(&state.db)
            .await
        } else {
            sqlx::query_as!(
                IngredientItem,
                "SELECT id, name, creator_id FROM ingredient_items ORDER BY name ASC OFFSET $1 LIMIT $2",
                start,
                end
            )
            .fetch_all(&state.db)
            .await
        }
    }
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get ingredient items: {}", e)))?;

    Ok(Json(IngredientItemsResponse {
        ingredient_items: query,
    }))
} 

pub async fn delete_ingredient_item(
    Extension((user, _token)): Extension<(User, String)>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Only allow deletion if user is the creator
    let result = sqlx::query!(
        "DELETE FROM ingredient_items WHERE id = $1 AND creator_id = $2",
        id,
        user.id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete ingredient item: {}", e),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Ingredient item not found".to_string()));
    }

    Ok(StatusCode::OK)
}