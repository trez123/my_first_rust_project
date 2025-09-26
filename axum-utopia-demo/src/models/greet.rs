// Define a response struct
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct Greeting {
    pub message: String,
}
