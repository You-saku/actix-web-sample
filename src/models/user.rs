use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}
