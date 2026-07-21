use wasm_bindgen::prelude::*;
use image::{
    imageops::FilterType,
};
use std::io::Cursor;

#[wasm_bindgen]
pub fn compress_image(
    bytes: &[u8],
    width: u32,
    height: u32,
    quality: u8,
) -> Vec<u8> {

    let img = image::load_from_memory(bytes)
        .expect("Unable to decode image");

    let resized = img.resize(
        width,
        height,
        FilterType::Lanczos3,
    );

    let mut output = Cursor::new(Vec::new());

    let mut encoder =
        image::codecs::jpeg::JpegEncoder::new_with_quality(
            &mut output,
            quality,
        );

    encoder
        .encode_image(&resized)
        .unwrap();

    output.into_inner()
}
