use axum::serve;
use std::net::SocketAddr;
use utoipa::OpenApi;

mod db;
mod handlers;
mod models;
mod routes;
mod schema;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::user_handler::create_user_handler,
        handlers::greet_handler::greet,
        handlers::user_handler::get_user_handler,
        handlers::user_handler::delete_user_handler,
        handlers::user_handler::get_all_users,
    ),
    components(schemas(models::user::NewUser, models::user::User)),
    tags(
        (name = "Greet", description = "Greeting endpoints"),
        (name = "Users", description = "User management endpoints")
    ),
    info(title = "Axum Diesel Example API", version = "1.0.0",)
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = db::create_pool().await?;
    let api_doc = ApiDoc::openapi();
    let app = routes::create_router(pool, api_doc);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    serve(listener, app).await?;
    Ok(())
}
