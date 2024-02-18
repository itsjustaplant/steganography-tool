use image::DynamicImage;
use std::str;

pub fn decode(image: &DynamicImage, _key: &str) {
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
        Ok(message) => println!("{}", message),
        Err(_) => println!("Couldn't extract the message from the image")
    }
}
