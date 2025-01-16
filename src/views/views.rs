use rocket::{Route, get, routes};
use rocket_dyn_templates::{Template, context};

// ENDPOINTS
#[get("/")]
fn home_page() -> Template {
    Template::render("index", context! {
        title: "Todo List",
        message: "Welcome!"
    })
}

// FUNCTIONS
pub fn routes() -> Vec<Route> {
    routes![home_page]
}
