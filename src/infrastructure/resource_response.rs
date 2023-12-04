use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceResponse<T> {
    data: T,
}

impl<T> ResourceResponse<T> {
    pub fn new(data: T) -> Json<Self> {
        Json(ResourceResponse { data })
    }
}
