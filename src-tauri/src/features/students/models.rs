use serde::{Deserialize, Serialize};
use sqlx::FromRow;

//estructura de los datos que enviamos y recibimos a la base de datos
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Student {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub created_at: String,
}

// lo que recibiremos al crear Student
#[derive(Debug, Deserialize)]
pub struct CreateStudent {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudent {
    pub first_name: String,
    pub last_name: String,
}
