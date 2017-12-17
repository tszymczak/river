// A Rust program that prints images to the terminal. A fun, if useless,
// project that taught me to code in Rust.
use std::path::Path;
use std::process;
extern crate image;
use image::{GenericImage, Pixel, FilterType};
extern crate clap;
use clap::{App, Arg};


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
        .arg(Arg::with_name("mode")
            .help("What visual style to use when printing the image.")
            .short("m")
            .takes_value(true)
            .possible_values(&["pound", "ascii"]))
       .get_matches();

    // Get the input file name. Crash if not specified.
    let infile_name;
    if matches.is_present("INPUT") {
        infile_name = matches.value_of("INPUT").unwrap();
    } else {
        println!("No input file name supplied!");
        process::exit(1);
    }

    // Handle mode inputs. If the user doesn't specify a mode, default to
    // pound.
    let mode = matches.value_of("mode").unwrap_or("pound");

    // Open the input image file and resize it.
    let inimg = image::open(&Path::new(&infile_name)).ok().expect("Opening image failed");
    let img = inimg.resize_exact(80, 24, FilterType::Nearest);

    // Render the image to the terminal.
    render(img, mode);
}


// Print the input image file.
fn render(img: image::DynamicImage, mode: &str) {
    if mode == "pound" {
        render_pound(img);
    } else if mode == "ascii" {
        render_ascii(img);
    } else {
        println!("Invalid rendering mode {}.", mode);
        process::exit(1);
    }
}

fn render_pound(img: image::DynamicImage) {
    let (width, height) = img.dimensions();
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

fn render_ascii(img: image::DynamicImage) {
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            let this_pixel = img.get_pixel(x, y);
            let lum = this_pixel.to_luma();
            let luma = lum.data[0];
            if luma < 64 {
                print!("0");
            } else if luma < 128 {
                print!(":");
            } else if luma < 192 {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
