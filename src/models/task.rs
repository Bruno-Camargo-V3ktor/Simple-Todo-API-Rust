use rocket::serde::{Deserialize, Serialize};

// Structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
