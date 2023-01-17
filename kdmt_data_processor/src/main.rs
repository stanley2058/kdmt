mod log_matcher;
mod route;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![route::api::create_log])
}
