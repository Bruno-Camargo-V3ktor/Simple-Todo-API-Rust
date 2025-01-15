use rocket::{launch, routes};

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![])
}
