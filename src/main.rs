use image;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

fn main() {
    let img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |_x, _y| image::Rgb([0u8; 3]));

    img.save("avatar.png").unwrap();
}
