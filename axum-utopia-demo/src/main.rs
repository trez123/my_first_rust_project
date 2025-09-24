use axum::{
    routing::get,
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Define your API schema with utoipa macros
#[derive(OpenApi)]
#[openapi(
    paths(greet),
    components(schemas(Greeting)),
    tags(
        (name = "greet", description = "Greeting API")
    )
)]
struct ApiDoc;

// Define a response struct
#[derive(serde::Serialize, utoipa::ToSchema)]
struct Greeting {
    message: String,
}

// Define a handler function with utoipa documentation
#[utoipa::path(
    get,
    path = "/greet",
    responses(
        (status = 200, description = "Greet someone", body = Greeting)
    )
)]
async fn greet() -> axum::Json<Greeting> {
    axum::Json(Greeting {
        message: "Hello from Axum + utoipa!".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/greet", get(greet))
        // Serve the Swagger UI at /swagger-ui endpoint
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    // Run server
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
