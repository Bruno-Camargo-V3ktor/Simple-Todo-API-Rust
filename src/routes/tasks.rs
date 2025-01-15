use crate::db::mem_db::Db;
use crate::models::task::Task;
use rocket::serde::json::Json;
use rocket::{Route, State, get, routes};

// ENDPOINTS
#[get("/")]
pub fn list_task(db: &State<Db>) -> Json<Vec<Task>> {
    let db = db.lock().expect("Falid load db");
    let tasks: Vec<Task> = db.values().cloned().collect();
    Json(tasks)
}

// FUNCTIONS
pub fn routes() -> Vec<Route> {
    routes![list_task]
}
