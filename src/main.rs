// Steps to do this:
// 1. Open file
// 2. Read image
// 3. Resize image
// 4. Render image
use std::io;
use std::path::Path;
extern crate image;
use image::{GenericImage, Pixel, FilterType, ImageResult, ImageBuffer, DynamicImage};

fn main() {
    // Get the input file name.
    let infile_name = read_infile_name();

    // Open the input image file and resize it.
    let inimg = image::open(&Path::new(&infile_name)).ok().expect("Opening image failed");
    let img = inimg.resize(32, 32, FilterType::Nearest);

    // Render the image to the terminal.
    render(img);
}

fn read_infile_name() -> String {
    // Read the input file name from the user.
    let mut infile_name = String::new();
    println!("Enter input file name:");
    io::stdin().read_line(&mut infile_name).expect("Failed to read input file name.");
    println!("Reading input from {}", infile_name);

    // Cut the newline off the read string.
    // Source: https://stackoverflow.com/questions/37888042/remove-single-trailing-newline-from-string-without-cloning
    let new_length = infile_name.trim_right().len();
    infile_name.truncate(new_length);
    return infile_name;
}

// Print the input image file.
fn render(img: image::DynamicImage) {
    let (width, height) = img.dimensions();
    println!("{} {}", width, height);
    for y in 0..height {
        for x in 0..width {
            let this_pixel = img.get_pixel(x, y);
            let lum = this_pixel.to_luma();
            let luma = lum.data[0];
            //println!("{}", brightness);
            if luma < 128 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
