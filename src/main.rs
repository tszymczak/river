// Steps to do this:
// 1. Open file
// 2. Read image
// 3. Resize image
// 4. Render image
use std::io;
use std::path::Path;
extern crate image;
use image::{GenericImage, Pixel, FilterType};
extern crate clap;
use clap::{App, Arg};
use std::process;


fn main() {
    // Parse command line input.
    let matches = App::new("River")
        .version("0.1")
        .about("Print images in the Terminal using text characters.")
        .author("Thomas Szymczak")
        .arg(Arg::with_name("INPUT")
            .help("The name of the input file")
            .required(true)
            .index(1))
        .arg(Arg::with_name("style")
            .help("What style to use when printing the image. This determines how it looks.")
            .short("s")
            .possible_values(&["pound"]))
       .get_matches();

    // Get the input file name.
    let mut infile_name = "";
    if matches.is_present("INPUT") {
        println!("Input file is {}", matches.value_of("INPUT").unwrap());
        infile_name = matches.value_of("INPUT").unwrap();
    } else {
        println!("No input file name supplied!");
        infile_name = "";
        process::exit(1);
    }

    // Open the input image file and resize it.
    let inimg = image::open(&Path::new(&infile_name)).ok().expect("Opening image failed");
    let img = inimg.resize(32, 32, FilterType::Nearest);

    // Render the image to the terminal.
    render(img);
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
