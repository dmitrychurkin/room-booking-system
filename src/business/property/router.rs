use rocket::{*, serde::json::Json};
use rocket_db_pools::Connection;

use crate::infrastructure::{db::RoomBookingSystem, resource_response::ResourceResponse};

use super::{controller, model::Property};

#[get("/properties")]
pub async fn index(db: Connection<RoomBookingSystem>) -> Option<Json<ResourceResponse<Vec<Property>>>> {
    controller::index(db).await
}
