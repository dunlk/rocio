use sqlx::SqlitePool;
use tauri::State;

use super::models::*;
use super::repository;

#[tauri::command]
pub async fn get_activities(pool: State<'_, SqlitePool>) -> Result<Vec<Activity>, String> {
    repository::get_all_activities(&pool)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn create_activity(
    pool: State<'_, SqlitePool>,
    data: CreateActivity,
) -> Result<Activity, String> {
    repository::create_activity(&pool, data)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn update_activity(
    pool: State<'_, SqlitePool>,
    id: i64,
    data: UpdateActivity,
) -> Result<Activity, String> {
    repository::update_activity(&pool, data)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn delete_activity(pool: State<'_, SqlitePool>, id: i64) -> Result<(), String> {
    repository::delete_activity(&pool, id)
        .await
        .map_err(|error| error.to_string())
}
