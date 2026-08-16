use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Activity {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub activity_type: String,
    pub amount: f64,
    pub activities_date: String,
    pub due_date: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateActivity {
    pub name: String,
    pub description: Option<String>,
    pub activity_type: Option<String>,
    pub amount: f64,
    pub activities_date: String,
    pub due_date: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateActivity {
    pub name: Option<String>,
    pub description: Option<String>,
    pub activity_type: Option<String>,
    pub amount: Option<f64>,
    pub activities_date: Option<String>,
    pub due_date: Option<String>,
}
