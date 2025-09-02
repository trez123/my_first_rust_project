use wasm_bindgen::prelude::*;
use photon_rs::native::{open_image_from_bytes, image_to_bytes};
use photon_rs::transform::{crop, resize, SamplingFilter};

#[wasm_bindgen]
pub fn process_image(input_bytes: &[u8]) -> Vec<u8> {
    let mut img = open_image_from_bytes(input_bytes).expect("Failed to open image");

    let (width, height) = (img.get_width(), img.get_height());

    let scale = 800.0 / width.min(height) as f32;
    let new_width = (width as f32 * scale) as u32;
    let new_height = (height as f32 * scale) as u32;
    let mut img = resize(&mut img, new_width, new_height, SamplingFilter::Lanczos3);

    let crop_x = (new_width - 800) / 2;
    let crop_y = (new_height - 800) / 2;
    let img = crop(&mut img, crop_x, crop_y, 800, 800);
    image_to_bytes(img)
}