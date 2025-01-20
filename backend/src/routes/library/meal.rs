use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, Extension, Json};
use backend::util::{app_state::AppState, user::User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct MealItemCreationInformation {
    name: String,
    ingredients: Vec<Uuid>,
    instructions: String,
}

pub async fn create_meal_item(
    Extension((user, _token)): Extension<(User, String)>,
    State(state): State<AppState>,
    Json(meal_item): Json<MealItemCreationInformation>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!(
        "INSERT INTO meal_items (name, ingredient_items, instructions, creator_id) VALUES ($1, $2, $3, $4)",
        meal_item.name,
        &meal_item.ingredients,
        meal_item.instructions,
        user.id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create meal item: {}", e),
        )
    })?;
    Ok(StatusCode::OK)
}

#[derive(Serialize)]
pub struct MealItems {
    pub id: Uuid,
    pub name: String,
    pub creator_id: Uuid,
}
#[derive(Serialize)]
pub struct MealItemsResponse {
    meal_items: Vec<MealItems>,
}
#[derive(Deserialize)]
pub struct GetMealItemsInformation {
    start: Option<i64>,
    end: Option<i64>,
    search: Option<String>,
}

pub async fn get_meal_items(
    state: State<AppState>,
    Query(params): Query<GetMealItemsInformation>,
) -> Result<Json<MealItemsResponse>, (StatusCode, String)> {
    let start = params.start.unwrap_or(0);
    let end = params.end.unwrap_or(10);

    let query: Vec<MealItems> = if let Some(search) = params.search {
        sqlx::query_as!(
            MealItems,
            "SELECT id, name, creator_id FROM meal_items WHERE name ILIKE $1 ORDER BY name ASC OFFSET $2 LIMIT $3",
            format!("%{}%", search),
            start,
            end
        )
        .fetch_all(&state.db)
        .await
    } else {
        sqlx::query_as!(
            MealItems,
            "SELECT id, name, creator_id FROM meal_items ORDER BY name ASC OFFSET $1 LIMIT $2",
            start,
            end
        )
        .fetch_all(&state.db)
        .await
    }
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get meal items: {}", e)))?;

    Ok(Json(MealItemsResponse { meal_items: query }))
}

#[derive(Serialize)]
pub struct MealItem {
    pub id: Uuid,
    pub name: String,
    pub ingredient_items: Vec<FoodItem>,
    pub instructions: String,
}
#[derive(Serialize, Deserialize)]
pub struct FoodItem {
    pub id: Uuid,
    pub name: String,
}

pub async fn get_meal_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MealItem>, (StatusCode, String)> {
    let meal = sqlx::query!(
        r#"
        SELECT m.id, m.name, m.instructions,
        (
            SELECT COALESCE(
                json_agg(json_build_object('id', f.id, 'name', f.name)), 
                '[]'::json
            )
            FROM ingredient_items f
            WHERE f.id = ANY(m.ingredient_items)
        ) as "ingredient_items!"
        FROM meal_items m
        WHERE m.id = $1
        "#,
        id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::NOT_FOUND, format!("Meal not found: {}", e)))?;

    Ok(Json(MealItem {
        id: meal.id,
        name: meal.name,
        ingredient_items: serde_json::from_value(meal.ingredient_items).unwrap(),
        instructions: meal.instructions,
    }))
}

pub async fn delete_meal_item(
    Extension((user, _token)): Extension<(User, String)>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!(
        "DELETE FROM meal_items WHERE id = $1 AND creator_id = $2",
        id,
        user.id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete meal item: {}", e),
        )
    })?;

    Ok(StatusCode::OK)
}
