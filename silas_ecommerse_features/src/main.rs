use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::hello,
        routes::echo,
        routes::manual_hello
    ),
    components(
        schemas(routes::EchoRequest)
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Register all routes from the routes module
            .configure(routes::configure)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}