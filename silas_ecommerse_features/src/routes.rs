// use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder};
// use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
// use std::time::Instant;
use utoipa::ToSchema;
// use vips::ops;
// use vips::VipsImage;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EchoRequest {
    content: String,
}
// #[derive(utoipa::ToSchema)]
// struct UploadFile {
//     #[schema(content_media_type = "application/octet-stream")]
//     file_bytes: Vec<u8>,
// }

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Hello world response", body = String)
    )
)]
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[utoipa::path(
    post,
    path = "/echo",
    request_body(content = String, description = "Request body as string"),
    responses(
        (status = 200, description = "Echo response", body = String)
    )
)]
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[utoipa::path(
    get,
    path = "/hey",
    responses(
        (status = 200, description = "Manual hello response", body = String)
    )
)]
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// #[utoipa::path(
//     post,
//     path = "/process-image-vips",
//     request_body(content = UploadFile, content_type = "multipart/form-data"),
//     responses(
//         (status = 200, description = "The processed image.", body = Vec<u8>, content_type = "image/jpeg", headers(
//             ("X-Processing-Time-Ms" = u64, description = "Time taken to process the image in milliseconds")
//         )),
//         (status = 400, description = "Bad Request. No file uploaded or invalid file.")
//     )
// )]
// #[post("/process-image-vips")]
// pub async fn process_image_vips(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
//     let start = Instant::now();

//     let mut file_data: Option<Vec<u8>> = None;

//     while let Some(mut field) = payload.try_next().await? {
//         if field.name() == "file" {
//             let mut data = Vec::new();
//             while let Some(chunk) = field.try_next().await? {
//                 data.extend_from_slice(&chunk);
//             }
//             if !data.is_empty() {
//                 file_data = Some(data);
//             }
//             break; // Found the file field, exit loop
//         }
//     }

//     let file_data =
//         file_data.ok_or_else(|| actix_web::error::ErrorBadRequest("File not found in multipart data"))?;

//     let image = VipsImage::new_from_buffer(&file_data, "")
//         .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to load image: {}", e)))?;

//     // Equivalent to sharp's resize(800, 800, { fit: 'cover', position: 'left top' })
//     let thumbnail = ops::thumbnail_with_opts(
//         &image,
//         800,
//         &ops::ThumbnailOptions {
//             height: 800,
//             crop: vips::Interesting::Low, // 'left top'
//             ..ops::ThumbnailOptions::default()
//         },
//     )
//     .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to process image: {}", e)))?;

//     let jpeg_buffer = ops::jpegsave_buffer(&thumbnail)
//         .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to save image as JPEG: {}", e)))?;

//     let duration_ms = start.elapsed().as_millis();

//     Ok(HttpResponse::Ok()
//         .content_type("image/jpeg")
//         .insert_header(("X-Processing-Time-Ms", duration_ms.to_string()))
//         .body(jpeg_buffer))
// }

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(echo)
        // .service(process_image_vips)
        .route("/hey", web::get().to(manual_hello));
}
