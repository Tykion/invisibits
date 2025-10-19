use std::io::Cursor;

use hound::{WavReader};

pub fn wav_decode(wav_data: &[u8] , password: Option<&str>) -> String {

    // convert wav file to sample bytes
    let mut reader = WavReader::new(Cursor::new(wav_data))
        .unwrap_or_else(|_| wasm_bindgen::throw_str("Invalid WAV file"));
    let samples: Vec<i16> = reader
        .samples::<i16>()
        .map(|s| s.unwrap())
        .collect();

    let mut password_len_bits = Vec::new();

    for sample in &samples[0..8] {
        let bit = (*sample as u16) & 1;
        password_len_bits.push(bit as u8);
    }
    let password_len = password_len_bits.iter().fold(0u8, |acc, &bit| (acc << 1) | bit); 

    let mut start_index = 8;

    if start_index + 8 > samples.len() {
        wasm_bindgen::throw_str("Corrupted or incomplete WAV data");
    }

    if password_len > 0 {
        let mut extracted_password = String::new();
        
        for _i in 0..password_len {
            let mut byte = 0u8;

            for i in 0..8 {
                let sample = samples[start_index + i];
                let bit = (sample as u16) & 1;
                byte = (byte << 1) | (bit as u8)
            }
            extracted_password.push(byte as char);
            start_index += 8;
        }
        if extracted_password != password.unwrap_or("").to_string() {
            wasm_bindgen::throw_str("Invalid password\nNote: could be unsupported encoding method")
        }
    }

    let mut message_len_bits = Vec::new();
    for sample in &samples[start_index..start_index + 8] {
        let bit = (*sample as u16) & 1;
        message_len_bits.push(bit as u8);
    }
    let message_len = message_len_bits.iter().fold(0u8, |acc, &bit| (acc << 1) | bit);
    start_index += 8;

    let mut extracted_message = String::new();
    
    for _i in 0..message_len {
        let mut byte = 0u8;

        for i in 0..8 {
            let sample = samples[start_index + i];
            let bit = (sample as u16) & 1;
            byte = (byte << 1) | (bit as u8)
        }
        extracted_message.push(byte as char);
        start_index += 8;
    }

    extracted_message
}