use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Estructura de los datos que recibimos/enviamos a la base de datos
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Activity {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub activity_type: String,
    pub amount: f64,
    pub activities_date: Option<String>,
    pub due_date: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateActivity {
    pub name: String,
    pub description: Option<String>,
    pub activity_type: String,
    pub amount: f64,
    pub activities_date: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateActivity {
    pub name: Option<String>,
    pub description: Option<String>,
    pub activity_type: Option<String>,
    pub amount: Option<f64>,
    pub activities_date: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ActivityPayments {
    pub id: i64,
    pub activity_id: i64,
    pub student_id: i64,
    pub amount_paid: f64,
    pub status: String,
    pub paid_at: String,
    pub created_at: String,
}
