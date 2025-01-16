mod db;
mod models;
mod routes;
mod views;

use db::mem_db::init_db;
use rocket::launch;
use rocket_dyn_templates::Template;

#[launch]
fn launch() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .manage(init_db()) // State of DB
        .mount("/", views::views::routes())
        .mount("/api/task", routes::tasks::routes())
}
