use sqlx::SqlitePool;

use crate::features::students::models::{CreateStudent, UpdateStudent};

use super::models::Student;

pub async fn get_all_students(pool: &SqlitePool) -> Result<Vec<Student>, sqlx::Error> {
    let students = sqlx::query_as::<_, Student>(
        r#"
        SELECT
            id,
            first_name,
            last_name,
            created_at
        FROM students
        ORDER BY id DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(students)
}

pub async fn create_student(
    pool: &SqlitePool,
    data: CreateStudent,
) -> Result<Student, sqlx::Error> {
    let student = sqlx::query_as::<_, Student>(
        r#"
        INSERT INTO students (
            first_name,
            last_name
        )
        VALUES (?, ?)
        RETURNING
            id,
            first_name,
            last_name,
            created_at
        "#,
    )
    .bind(data.first_name)
    .bind(data.last_name)
    .fetch_one(pool)
    .await?;

    Ok(student)
}

pub async fn update_student(
    pool: &SqlitePool,
    id: i64,
    data: UpdateStudent,
) -> Result<Student, sqlx::Error> {
    let student = sqlx::query_as::<_, Student>(
        r#"
            UPDATE students
            SET
                first_name = ?,
                last_name = ?
            WHERE id = ?
            RETURNING
                id,
                first_name,
                last_name
                created_at
        "#,
    )
    .bind(data.first_name)
    .bind(data.last_name)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(student)
}

pub async fn delete_student(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
            DELETE students
            WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}
