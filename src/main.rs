use image::GenericImageView;

pub mod cli;

fn main() {
    let (arg_1, arg_2, mode) = cli::init_cli();

    let img = image::open(&arg_1);

    let max_input_len = match img {
        Ok(img_result) => {
            let (x, y) = img_result.dimensions();
            x * y
        }
        Err(_) => 0u32,
    };

    match max_input_len {
        0 => panic!("Couldn't find the image"),
        _ => println!("{}", max_input_len),
    }

    println!("{} {} {:?}", arg_1, arg_2, mode);
}
