use axum::{
    Router,
    routing::{delete, get, post},
};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

use super::handlers::{
    greet::greet,
    user::{create_user_handler, delete_user_handler, get_all_users, get_user_handler},
};
use crate::{models, repositories::{state::AppState, user::UserRepository}};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::greet::greet,
        crate::handlers::user::create_user_handler, 
        crate::handlers::user::delete_user_handler, 
        crate::handlers::user::get_all_users, 
        crate::handlers::user::get_user_handler
    ),
    components(schemas(models::user::NewUser, models::user::User)),
    tags(
        (name = "Greet", description = "Greeting endpoints"),
        (name = "Users", description = "User management endpoints")
    ),
    info(title = "Axum Diesel Example API", version = "1.0.0",)
)]
struct ApiDoc;

// Define the correct async pool type
pub type DbPool = Pool<AsyncPgConnection>;

pub fn create_router(pool: DbPool) -> Router {
    let user_repo = UserRepository { pool };
    let api_doc = ApiDoc::openapi();
    let app_state =  AppState { user_repo };
    Router::new()
        // Greet handler is not implemented in the provided code, so let's assume it exists.
        .route("/greet", get(greet))
        .route("/users", get(get_all_users))
        .route("/users", post(create_user_handler))
        .route("/users/{id}", delete(delete_user_handler))
        .route("/users/{id}", get(get_user_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc))
        .with_state(app_state)
}
