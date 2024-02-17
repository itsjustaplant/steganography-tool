use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Mode {
    Encode,
    Decode,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Program mode encode | decode
    #[arg(value_enum, default_value_t = Mode::Encode)]
    mode: Mode,

    /// Path of the image and secret key || text to hide
    #[arg(last = true, required = true)]
    required_args: Vec<String>,
}

pub fn init_cli() -> (String, String, Mode) {
    let args = Args::parse();
    let required_args = args.required_args;
    let mode = args.mode;

    // File path
    let arg_1 = match required_args.get(0) {
        Some(v) => v,
        None => panic!("Image path should be provided"),
    };

    // encode => secret key
    // decode => text to insert into image
    let arg_2 = match required_args.get(1) {
        Some(v) => v,
        None => match mode {
            Mode::Encode => panic!("Secret key should be provided"),
            Mode::Decode => panic!("Text should be provided"),
        },
    };

    (arg_1.to_string(), arg_2.to_string(), mode)
}
