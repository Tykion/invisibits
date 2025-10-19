use std::io::Cursor;

use image::{load_from_memory, ImageFormat};
use wasm_bindgen::prelude::*;

mod decode;
mod encode;

mod wav_encode;
mod wav_decode;

const MAX_WAV_SIZE: usize = 50 * 1024 * 1024; // 50 MB

// wrappers, because js can't send RgbImage type directly so we need to transform it into raw data first and process raw data in rust

#[wasm_bindgen]
pub fn wav_encode_wrapper (
        message: &str,
        wav_data: &[u8],
        password: Option<String>) -> Vec<u8> {
    // added because wav files tend to be quite large
    if wav_data.len() > MAX_WAV_SIZE {
        wasm_bindgen::throw_str("WAV file too large");
    }
    let pwd_ref = password.as_deref();
    wav_encode::wav_encode(message, wav_data, pwd_ref)
}

#[wasm_bindgen]
pub fn wav_decode_wrapper (
        wav_data: &[u8],
        password: Option<String>) -> String {

    if wav_data.is_empty() {
        wasm_bindgen::throw_str("Empty WAV data");
    }
    
    let pwd_ref = password.as_deref();
    wav_decode::wav_decode(wav_data, pwd_ref)
}
    

#[wasm_bindgen]
pub fn encode_wrapper (
        message: &str,
        img_data: &[u8], // 8 bit array from the image instead
        password: Option<String>) -> Vec<u8> {
        
    let img = load_from_memory(img_data)
                        .unwrap_or_else(|_| wasm_bindgen::throw_str("Invalid image file"))
                        .to_rgb8();

    let encoded = encode::encode(message, img, password.as_deref());

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    encoded.write_to(&mut cursor, ImageFormat::Png)
                        .unwrap_or_else(|_| wasm_bindgen::throw_str("Failed to write image"));

    buf
}

#[wasm_bindgen]
pub fn decode_wrapper (
    img_data: &[u8],
    password: Option<String>) -> String {

    let img = image::load_from_memory(img_data)
                        .unwrap_or_else(|_| wasm_bindgen::throw_str("Invalid image file"))
                        .to_rgb8();
                    
    decode::decode(password.as_deref(), img)
}