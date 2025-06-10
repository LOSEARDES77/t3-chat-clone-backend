#[macro_use]
extern crate rocket;

mod api;
mod auth;
pub mod models;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![api::models])
}
