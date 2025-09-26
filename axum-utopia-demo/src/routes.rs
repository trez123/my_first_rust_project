use axum::{
    Router,
    routing::{delete, get, post},
};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool;
use utoipa::openapi::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers;

// Define the correct async pool type
pub type DbPool = Pool<AsyncPgConnection>;

pub fn create_router(pool: DbPool, api_doc: OpenApi) -> Router {
    Router::new()
        // Greet handler is not implemented in the provided code, so let's assume it exists.
        .route("/greet", get(handlers::greet_handler::greet))
        .route("/users", get(handlers::user_handler::get_all_users))
        .route("/users", post(handlers::user_handler::create_user_handler))
        .route(
            "/users/{id}",
            delete(handlers::user_handler::delete_user_handler),
        )
        .route("/users/{id}", get(handlers::user_handler::get_user_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc))
        .with_state(pool)
}
