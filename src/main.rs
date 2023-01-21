use clap::{App, Arg};
use image;

pub mod helpers;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

fn main() {
    let version = helpers::get_cargo_toml_version();

    let matches = App::new("Avatar image creator")
        .version(version.as_str())
        .author("Sarmat")
        .args_from_usage(
            "-w --width 'Image width'
            -h --height 'Image height'",
        )
        .arg(
            Arg::with_name("width")
                .short('w')
                .long("width")
                .value_name("WIDTH")
                .help("Sets the width of the image")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("height")
                .short('h')
                .long("height")
                .value_name("HEIGHT")
                .help("Sets the height of the image")
                .takes_value(true),
        )
        .get_matches();

    let width = matches
        .value_of("width")
        .unwrap_or(&WIDTH.to_string())
        .parse::<u32>()
        .unwrap();
    let height = matches
        .value_of("height")
        .unwrap_or(&HEIGHT.to_string())
        .parse::<u32>()
        .unwrap();

    let img = image::ImageBuffer::from_fn(width, height, |_x, _y| image::Rgb([0u8; 3]));

    img.save("avatar.png").unwrap();
    println!("✅ Image saved successfully as 'avatar.png'!");
}
