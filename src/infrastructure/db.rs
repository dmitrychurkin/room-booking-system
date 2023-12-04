use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("room-booking-system")]
pub struct RoomBookingSystem(sqlx::PgPool);
