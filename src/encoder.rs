use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

pub fn check_image_size(width: u32, height: u32, secret: &str) -> bool {
    width * height >= secret.len() as u32
}

pub fn encode(image: &DynamicImage, secret: &str) {
    // get dimensions
    let (width, height) = image.dimensions();

    let can_fit_word = check_image_size(width, height, secret);

    if !can_fit_word {
        panic!("Secret is too long for image");
    }

    let rgba_byte_array = image.to_rgba16();
    let secret_array = secret.as_bytes();
    let mut new_image = ImageBuffer::<Rgba<u16>, Vec<u16>>::new(width, height);

    /*
    x x x x -> 0 * 4 + x
    x x x x -> 1 * 4 + x
    index is y * width + x
     */

    for (x, y, pixel) in rgba_byte_array.enumerate_pixels() {
        let index = x + width * y;

        let new_pixel = if index < secret.len() as u32 {
            Rgba::<u16>([
                pixel[0],
                pixel[1],
                pixel[2],
                pixel[3] - secret_array[index as usize] as u16,
            ])
        } else {
            Rgba::<u16>([pixel[0], pixel[1], pixel[2], pixel[3]])
        };

        new_image.put_pixel(x, y, new_pixel);
    }

    match ImageBuffer::save(&new_image, "./result.png") {
        Ok(_) => println!("Image saved successfully"),
        Err(_) => panic!("Couldn't save the image"),
    };
}
