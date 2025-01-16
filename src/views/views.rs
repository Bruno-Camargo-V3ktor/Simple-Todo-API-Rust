use rocket::{Route, get, routes};
use rocket_dyn_templates::{Template, context};

// ENDPOINTS
#[get("/todo")]
fn todo_page() -> Template {
    Template::render("todo", context! {})
}

// FUNCTIONS
pub fn routes() -> Vec<Route> {
    routes![todo_page]
}
