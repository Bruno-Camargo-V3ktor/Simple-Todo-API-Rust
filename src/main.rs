mod db;
mod models;

use db::mem_db::init_db;
use rocket::{launch, routes};

#[launch]
fn launch() -> _ {
    rocket::build()
        .manage(init_db()) // State of DB
        .mount("/", routes![])
}
