use rocket::serde::json::Json;
use rocket_db_pools::{
    sqlx::{self},
    Connection,
};

use crate::infrastructure::{db::RoomBookingSystem, resource_response::ResourceResponse};

use super::model::Property;

// TODO: introduce service layer and move DB manipulation into repository
pub async fn index(
    mut db: Connection<RoomBookingSystem>,
) -> Option<Json<ResourceResponse<Vec<Property>>>> {
    sqlx::query_as("SELECT *, created_at::TEXT, updated_at::TEXT FROM properties")
        .fetch_all(&mut **db)
        .await
        .and_then(|rows| Ok(ResourceResponse::new(rows)))
        .ok()
}
