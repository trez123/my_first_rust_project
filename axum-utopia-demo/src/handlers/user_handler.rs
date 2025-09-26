use axum::http::StatusCode;
use axum::{
    Json,
    extract::{Path, State},
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::deadpool::Pool};

use crate::models::user::{NewUser, User};
use crate::schema::users;

// Define the correct async pool type alias
type DbPool = Pool<AsyncPgConnection>;

#[utoipa::path(
    post,
    path = "/users",
    summary = "Create a new user",
    tag = "Users",
    request_body = NewUser,
    responses(
        (status = 201, description = "User created", body = User),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_user_handler(
    State(pool): State<DbPool>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get DB connection: {}", e),
        )
    })?;

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    summary = "Get a user by ID",
    tag = "Users",
    params(("id" = i32, Path, description = "User ID")),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user_handler(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get DB connection: {}", e),
        )
    })?;

    users::table
        .find(id)
        .select(User::as_select())
        .first(&mut conn)
        .await
        .map(Json)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => (StatusCode::NOT_FOUND, "User not found".into()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        })
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    summary = "Delete a user by ID",
    tag = "Users",
    params(("id" = i32, Path, description = "User ID")),
    responses(
        (status = 200, description = "User deleted", body = usize),
        (status = 404, description = "User not found")
    )
)]
pub async fn delete_user_handler(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get DB connection: {}", e),
        )
    })?;

    let rows_deleted = diesel::delete(users::table.find(user_id))
        .execute(&mut conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if rows_deleted == 0 {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    }

    Ok(Json(rows_deleted))
}

#[utoipa::path(
    get,
    path = "/users",
    summary = "Get all users",
    tag = "Users",
    responses(
        (status = 200, description = "Users found", body = Vec<User>)
    ))]
pub async fn get_all_users(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get DB connection: {}", e),
        )
    })?;

    let results = users::table
        .select(User::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(results))
}
