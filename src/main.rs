use rocket::*;

mod app;
mod business;
mod infrastructure;

#[launch]
fn rocket() -> Rocket<Build> {
    app::init()
}
