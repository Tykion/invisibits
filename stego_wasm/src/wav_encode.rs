use std::io::Cursor;

// import hound crate to read and write wav files
use hound::{WavReader, WavWriter};

pub fn wav_encode(message: &str, wav_data: &[u8] , password: Option<&str>) -> Vec<u8> {
    // convert our inputs as bytes to calculate if the message and optional password can fit into our wav
    let message_bytes = message.as_bytes();
    let password_bytes = password.unwrap_or("").as_bytes();

    // convert message and password to bits iterator
    let message_bits: Vec<u8> = std::iter::once(password_bytes.len() as u8)
        .chain(password_bytes.iter().copied())
        .chain(std::iter::once(message_bytes.len() as u8))
        .chain(message_bytes.iter().copied())
        .flat_map(|byte| {
            format!("{:08b}", byte)
                .chars()
                .map(|c| c.to_digit(2).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect();


    let mut reader = WavReader::new(Cursor::new(wav_data))
        .unwrap_or_else(|_| wasm_bindgen::throw_str("Invalid WAV file"));
    let spec = reader.spec();
    let mut samples: Vec<i16> = reader
        .samples::<i16>()
        .map(|s| s.unwrap())
        .collect();
    
    if message_bits.len() > samples.len() {
        wasm_bindgen::throw_str("WAV file too short")
    }

    for (i, bit) in message_bits.iter().enumerate() {
        samples[i] = (samples[i] & !1) | *bit as i16;
    }

    let mut out_data = Vec::new();
    {
        let mut writer = WavWriter::new(Cursor::new(&mut out_data), spec)
            .unwrap_or_else(|_| wasm_bindgen::throw_str("Failed to create output WAV"));
        for s in samples {
            writer.write_sample(s).unwrap();
        }
        writer.finalize().unwrap();
    }

    out_data
}
