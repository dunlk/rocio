use crate::features::activities::models::{Activity, CreateActivity};

use super::repository;
use sqlx::SqlitePool;

pub async fn create_activity(
    pool: &SqlitePool,
    data: CreateActivity,
) -> Result<Activity, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let activity = repository::create_activity(&mut *tx, data).await?;

    repository::create_for_activity(&mut *tx, activity.id).await?;

    tx.commit().await?;

    Ok(activity)
}
