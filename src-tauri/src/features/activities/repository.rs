use sqlx::SqliteConnection;
use sqlx::SqlitePool;

use super::models::{Activity, CreateActivity, UpdateActivity};

pub async fn get_all_activities(pool: &SqlitePool) -> Result<Vec<Activity>, sqlx::Error> {
    let actitivites = sqlx::query_as::<_, Activity>(
        r#"
        SELECT
            id,
            name,
            description,
            activity_type,
            amount,
            activities_date,
            due_date,
            created_at
        FROM 
            activities
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(actitivites)
}

pub async fn create_activity(
    connection: &mut SqliteConnection,
    data: CreateActivity,
) -> Result<Activity, sqlx::Error> {
    let activity = sqlx::query_as::<_, Activity>(
        r#"
        INSERT INTO activities (
            name,
            description,
            activity_type,
            amount,
            activities_date,
            due_date
        )
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING
            id,
            name,
            description,
            activity_type,
            amount,
            activities_date,
            due_date,
            created_at
        "#,
    )
    .bind(data.name)
    .bind(data.description)
    .bind(data.activity_type)
    .bind(data.amount)
    .bind(data.activities_date)
    .bind(data.due_date)
    .fetch_one(connection)
    .await?;

    Ok(activity)
}

pub async fn update_activity(
    pool: &SqlitePool,
    id: i64,
    data: UpdateActivity,
) -> Result<Activity, sqlx::Error> {
    let activity = sqlx::query_as::<_, Activity>(
        r#"
        UPDATE activities
            name = ?,
            description = ?,
            activity_type = ?,
            amount = ?,
            activities_date = ?,
            due_date = ?
        WHERE ID = ?
        RETURNING
            id,
            name,
            description,
            activity_type,
            amount,
            activities_date,
            due_date,
            created_at
        "#,
    )
    .bind(data.name)
    .bind(data.description)
    .bind(data.activity_type)
    .bind(data.amount)
    .bind(data.activities_date)
    .bind(data.due_date)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(activity)
}

pub async fn delete_activity(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE activities
        WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_for_activity(
    tx: &mut sqlx::SqliteConnection,
    activity_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO activity_payments (
            activity_id,
            student_id
        )
        SELECT
            ?,
            id
        FROM students
        "#,
    )
    .bind(activity_id)
    .execute(tx)
    .await?;

    Ok(())
}
