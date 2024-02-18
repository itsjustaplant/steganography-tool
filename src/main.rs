pub mod cli;
pub mod decoder;
pub mod encoder;

fn main() {
    let (arg_1, arg_2, mode) = cli::init_cli();
    let img = image::open(arg_1);

    match img {
        Ok(img_result) => match mode {
            cli::Mode::Encode => {
                encoder::encode(&img_result, arg_2.as_str());
            }
            cli::Mode::Decode => {
                decoder::decode(&img_result, arg_2.as_str());
            }
        },
        Err(_) => panic!("Couldn't find the image"),
    };
}
