use crate::models::greet::Greeting;

#[utoipa::path(
    get,
    path = "/greet",
    summary = "Greet someone",
    tag = "Greet",
    responses(
        (status = 200, description = "Greet someone", body = Greeting)
    )
)]
pub async fn greet() -> axum::Json<Greeting> {
    axum::Json(Greeting {
        message: "Hello from Axum + utoipa!".to_string(),
    })
}
