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
// (width/height) of the terminal characters and the maximum size. TODO:
// Autodetect size and make the size configurable.
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
    if aspect > 1.0 {
        let xeff: f32 = xi as f32;
        let yeff: f32 = (yi as f32) * aspect;
        let xscale: f32 = (xmax as f32) / xeff;
        let yscale: f32 = (ymax as f32) / yeff;
        let scale: f32;
        if xscale < yscale {
            scale = xscale;
        } else {
            scale = yscale;
        }
        let xf: u32 = f32::trunc(xeff*scale) as u32;
        let yf: u32 = f32::trunc(yeff*scale) as u32;
        // Use nearest neighbor resizing to make it as sharp as possible.
        return inimg.resize_exact(xf, yf, FilterType::Nearest);
    } else if aspect < 1.0 {
        let xeff: f32 = (xi as f32) / aspect;
        let yeff: f32 = yi as f32;
        let xscale: f32 = (xmax as f32) / xeff;
        let yscale: f32 = (ymax as f32) / yeff;
        let scale: f32;
        if xscale < yscale {
            scale = xscale;
        } else {
            scale = yscale;
        }
        let xf: u32 = f32::trunc(xeff*scale) as u32;
        let yf: u32 = f32::trunc(yeff*scale) as u32;
        return inimg.resize_exact(xf, yf, FilterType::Nearest);
    } else {
        return inimg.resize(xmax, ymax, FilterType::Nearest);
    }
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
            // Get the brightness level of the pixel.
            let luma = img.get_pixel(x, y).to_luma().data[0];
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
            // Get the brightness level of the pixel.
            let luma = img.get_pixel(x, y).to_luma().data[0];
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
