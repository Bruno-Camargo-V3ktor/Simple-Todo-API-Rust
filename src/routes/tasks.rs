use crate::db::mem_db::Db;
use crate::models::task::Task;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Route, State, get, post, put, routes};

// ENDPOINTS
#[get("/")]
pub fn list_task(db: &State<Db>) -> Json<Vec<Task>> {
    let db = db.lock().expect("Falid load db");
    let tasks: Vec<Task> = db.values().cloned().collect();
    Json(tasks)
}

#[post("/", format = "application/json", data = "<data>")]
pub fn add_task(
    db: &State<Db>,
    data: Json<Task>,
) -> Result<status::Custom<()>, status::BadRequest<&'static str>> {
    let mut db = db.lock().expect("Falid load db");
    let mut task = data.into_inner();

    let new_id = db.keys().max().unwrap_or(&0) + 1;
    task.id = Some(new_id);

    db.insert(new_id, task);

    Ok(status::Custom(Status::Created, ()))
}

#[put("/<id>", format = "application/json", data = "<data>")]
pub fn update_task(
    db: &State<Db>,
    id: usize,
    data: Json<Task>,
) -> Result<status::Custom<()>, status::BadRequest<&'static str>> {
    let mut db = db.lock().expect("Falid load db");
    let task = data.into_inner();

    match db.get_mut(&id) {
        Some(t) => {
            t.title = task.title;
            t.description = task.description;
            t.completed = task.completed;
        }

        None => return Err(status::BadRequest("Task Not Found")),
    }

    Ok(status::Custom(Status::Ok, ()))
}

// FUNCTIONS
pub fn routes() -> Vec<Route> {
    routes![list_task, add_task, update_task]
}
