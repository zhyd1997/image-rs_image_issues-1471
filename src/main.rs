use image::{ImageBuffer};

fn run_example() {
    let w = 200;
    let h = 200;

    let img = ImageBuffer::from_fn(w, h, |x, y| {
        if (x + y) % 3 == 0 {
            image::Rgba([0, 0, 0, 255 as u16]) // TAKE CARE!
        } else {
            image::Rgba([255, 255, 255, 255 as u16]) // TAKE CARE!
        }
    });

    img.save("out.png").unwrap();
}

fn main() {
    // println!("Hello, world!");
    run_example();
}
