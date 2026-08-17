use sqlx::SqliteConnection;

//crea pago pendiente a todos los alumnos registrados hasta el momento
pub async fn create_for_activity(
    connection: &mut SqliteConnection,
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
    .execute(connection)
    .await?;

    Ok(())
}
