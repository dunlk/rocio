use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Student {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateStudent {
    pub first_name: String,
    pub last_name: String,
}
