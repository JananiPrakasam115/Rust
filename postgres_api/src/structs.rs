use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Customers {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub position: String,
    pub is_active: bool,
}
