mod db;
mod models;
mod routes;

use db::mem_db::init_db;
use rocket::launch;

#[launch]
fn launch() -> _ {
    rocket::build()
        .manage(init_db()) // State of DB
        .mount("/api/task", routes::tasks::routes())
}
