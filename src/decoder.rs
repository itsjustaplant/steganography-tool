use image::DynamicImage;
use std::str;

pub fn decode(image: &DynamicImage, _key: &str) -> Option<String> {
    let rgba_byte_array = image.to_rgba16();
    let mut secret: Vec<u8> = Vec::new();

    for (_, _, pixel) in rgba_byte_array.enumerate_pixels() {
      let character = u16::MAX - pixel[3];
      if character > 0 {
        secret.push((u16::MAX - pixel[3]) as u8);
      } else {
        break;
      }
    }

    match str::from_utf8(&secret) {
        Ok(message) => {
          println!("Secret: {}", message.to_string());
          return Some(message.to_string());
        },
        Err(_) => {
          println!("Couldn't decode the image");
          return None;
        }
    };
}
