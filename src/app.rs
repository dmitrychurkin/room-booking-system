use rocket::{self, routes, Build, Rocket};
use rocket_db_pools::Database;

use crate::{business::property, infrastructure::db::RoomBookingSystem};

pub fn init() -> Rocket<Build> {
    rocket::build()
        .attach(RoomBookingSystem::init())
        .mount("/api", routes![property::router::index])
}
