use rocket::*;

use room_booking_system::init;

#[launch]
fn rocket() -> Rocket<Build> {
    init()
}
