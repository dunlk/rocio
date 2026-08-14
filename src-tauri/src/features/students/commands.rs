use sqlx::SqlitePool;
use tauri::State;

use crate::features::students::{models::UpdateStudent, repository};

use super::models::{CreateStudent, Student};

#[tauri::command]
pub async fn get_students(pool: State<'_, SqlitePool>) -> Result<Vec<Student>, String> {
    repository::get_all_students(&pool)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn create_student(
    pool: State<'_, SqlitePool>,
    data: CreateStudent,
) -> Result<Student, String> {
    repository::create_student(&pool, data)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn update_student(
    pool: State<'_, SqlitePool>,
    id: i64,
    data: UpdateStudent,
) -> Result<Student, String> {
    repository::update_student(&pool, id, data)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn delete_student(pool: State<'_, SqlitePool>, id: i64) -> Result<(), String> {
    repository::delete_student(&pool, id)
        .await
        .map_err(|error| error.to_string())
}
