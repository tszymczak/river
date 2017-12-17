// A Rust program that prints images to the terminal. A fun, if useless,
// project that taught me to code in Rust.
use std::path::Path;
use std::process;
use std::f32;
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
    let img = resize(inimg);

    // Render the image to the terminal.
    render(img, mode);
}

// Resize an image for display in the terminal, based on the aspect ratio
// (width/height) of the terminal characters and the maximum size. Currently
// this function accomplishes its task in two resize operations which is
// inefficient and potentially accurate. TODO: Autodetect size and make it
// configurable, make it use only one resize operation.
fn resize(inimg: image::DynamicImage) -> image::DynamicImage {
    // This is the aspect ratio of each terminal character, equal to its width
    // divided by its height. This value is a good approximation for GNOME
    // Terminal on Linux but other platforms have different values.
    let aspect = 0.5;
    // Resize to fit a standard 80x24 terminal.
    let xmax: u32 = 80;
    let ymax: u32 = 24;
    let (width, height) = inimg.dimensions();
    let xi: u32 = width as u32;
    let yi: u32 = height as u32;
    // Use nearest neighbor resizing to make it as sharp as possible.
    // First resize to compensate for terminal characters not being square.
    // strimg = stretched imange.
    let strimg;
    let strx: u32;
    let stry: u32;
    if aspect > 1.0 {
        stry = f32::trunc((yi as f32)*aspect) as u32;
        strimg = inimg.resize_exact(xi, stry, FilterType::Nearest);
    } else if aspect < 1.0 {
        let inv_aspect = 1.0/aspect;
        strx = f32::trunc((xi as f32)*inv_aspect) as u32;
        strimg = inimg.resize_exact(strx, yi, FilterType::Nearest);
    } else {
        strimg = inimg;
    }

    // Finally, resize to fit the terminal.
    strimg.resize(xmax, ymax, FilterType::Nearest)
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
