use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Debug, Deserialize, utoipa::ToSchema)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

// For creating a new user - no id or created_at (auto-generated)
#[derive(Insertable, Deserialize, utoipa::ToSchema)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

// For updating user fields (partial updates)
#[derive(AsChangeset, Deserialize, utoipa::ToSchema)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
}
