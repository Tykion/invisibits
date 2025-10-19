use image::RgbImage;

pub fn decode(password: Option<&str>, image: RgbImage) -> String {
    let mut message_bits: Vec<u8> = Vec::new(); // img pixel vector

    for pixel in image.pixels() {
        message_bits.push(pixel.0[0] & 1);
        message_bits.push(pixel.0[1] & 1);
        message_bits.push(pixel.0[2] & 1);
    }
    // remove spaces
    let message_bits = message_bits;

    let password_len_bits = &message_bits[0..8];
    // convert slice of string to unsigned integer 8bit (initial_value, |accumulator, element| some_operation)
    let password_len = password_len_bits.iter().fold(0u8, |acc, &bit| (acc << 1) | bit); 
    // password start and end indexes
    let mut start_index = 8;

    // IF PASSWORD IS PRESENT
    if password_len > 0 {
        let mut extracted_password = String::new();
    // iterate over image bytes based on password length 
        for _i in 0..password_len {
            let byte_bits = &message_bits[start_index..start_index+8];
            let byte = byte_bits.iter().fold(0u8, |acc,&bit| (acc << 1) | bit);
            extracted_password.push(byte as char);
            start_index += 8;
        }
        println!("{}", extracted_password);
        if extracted_password != password.unwrap_or("").to_string() {
            // throw an error in browser console instead of panicking
            wasm_bindgen::throw_str("Invalid password\nNote: could be unsupported encoding method");
        } 
    }

    let message_len_bits = &message_bits[start_index..start_index+8];
        let message_len = message_len_bits.iter().fold(0u8, |acc, &bit| (acc << 1) | bit);
        start_index += 8;  

        let mut message = String::new(); // String to store message

        for _i in 0..message_len {
            let byte_bits = &message_bits[start_index..start_index+8];
            let byte = byte_bits.iter().fold(0u8, |acc,&bit| (acc << 1) | bit);
            message.push(byte as char);
            start_index += 8;
            
        }


    message 
}
