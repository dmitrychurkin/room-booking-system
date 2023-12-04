use rocket::serde::Serialize;
use rocket_db_pools::sqlx::{postgres::PgRow, Error, FromRow, Row};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Property {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl<'a> FromRow<'a, PgRow> for Property {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Property {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}
