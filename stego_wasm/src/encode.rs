// import image crate
use image::RgbImage;

pub fn encode(message: &str, mut image: RgbImage, password: Option<&str>) -> RgbImage {
    // convert our inputs as bytes to calculate if the message and optional password can fit into our image
    let message_bytes = message.as_bytes();
    let password_bytes = password.unwrap_or("").as_bytes();

    // convert message and password to bits iterator
    let mut message_bits = std::iter::once(password_bytes.len() as u8) // password length header
        .chain(password.unwrap_or("").chars().map(|c| c as u8)) // password bytes
        .chain(std::iter::once(message_bytes.len() as u8)) // message length header
        .chain(message.chars().map(|c| c as u8)) // message bytes
        .flat_map(|byte| {
            // convert each byte to bits
            format!("{:08b}", byte).chars().collect::<Vec<char>>()
        });

    // check if the message fits into the image
    let total_bits_needed = (password_bytes.len() + 1 + message_bytes.len() + 1) * 8; // 1 byte for password len, 1 for message len
    let total_image_bits = image.width() * image.height() * 3; // 3 bits per pixel

    if total_image_bits >= total_bits_needed as u32 {
        for pixel in image.pixels_mut() {
            // if we have more bit in message -> continue
            if let Some(bit) = message_bits.next() {
                let red = pixel.0[0] & 0b11111110 | bit.to_digit(2).unwrap() as u8;
                // pixel.0[0] -> pixel pixel.[R,G,B][0 -> R]
                // 0b11111110 to change only the last bit of specific channel
                pixel.0[0] = red
            }
            if let Some(bit) = message_bits.next() {
                let green = pixel.0[1] & 0b11111110 | bit.to_digit(2).unwrap() as u8;
                pixel.0[1] = green
            }
            if let Some(bit) = message_bits.next() {
                let blue = pixel.0[2] & 0b11111110 | bit.to_digit(2).unwrap() as u8;
                pixel.0[2] = blue
            }
        }
    } else {
        wasm_bindgen::throw_str("Image is too small for the message");
    }

    // return the image
    image
}
