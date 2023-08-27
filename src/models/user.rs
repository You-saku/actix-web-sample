use serde::{Deserialize, Serialize};
use sqlx::FromRow; // sqlx

#[derive(FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
    //#[serde(rename = "createdAt")]
    //pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    //#[serde(rename = "updatedAt")]
    //pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
