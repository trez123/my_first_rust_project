use axum::serve;
use std::net::SocketAddr;
mod db;
mod handlers;
mod models;
mod repositories;
mod routes;
mod schema;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = db::create_pool().await?;
    let app = routes::create_router(pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    serve(listener, app).await?;
    Ok(())
}
