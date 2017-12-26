// A Rust program that prints images to the terminal. A fun, if useless,
// project that taught me to code in Rust.
use std::path::Path;
use std::process;
use std::f32;
extern crate image;
use image::{GenericImage, Pixel, FilterType};
extern crate termion;
use termion::{color};
extern crate clap;
use clap::{App, Arg};



fn main() {
    // Parse command line input.
    let matches = App::new("River")
        .version("0.2")
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
            .possible_values(&["pound", "ascii", "8color"]))
       .get_matches();

    // Get the input file name. Crash if not specified.
    let infile_name;
    if matches.is_present("INPUT") {
        infile_name = matches.value_of("INPUT").unwrap();
    } else {
        println!("No input file name supplied!");
        process::exit(1);
    }

    // Get the width and height of the terminal (in characters).
    let (x, y) = get_dimensions();

    // Handle mode inputs. If the user doesn't specify a mode, default to
    // ascii.
    let mode = matches.value_of("mode").unwrap_or("ascii");

    // Open the input image file and resize it.
    let inimg = image::open(&Path::new(&infile_name)).ok().expect("Opening image failed");
    let img = resize(inimg, x, y);

    // Render the image to the terminal.
    render(img, mode);
}

// Get the dimensions of the current terminal, in characters. Returns a tuple
// containing the width of the terminal, then the height.
fn get_dimensions() -> (u32, u32) {
    let (default_x, default_y) = (80, 24);
    let term_result = termion::terminal_size();
    // If we can't get the size of the terminal, use 80x24 as a sane default.
    // This actually happens when we redirect the program's output.
    if term_result.is_ok() {
        let (x, y) = term_result.unwrap();
        return (x as u32, y as u32);
    } else {
        eprintln!("Warning: Can't get the dimensions of this terminal, assuming {} by {}", default_x, default_y);
        return (default_x, default_y);
    }
}

// Resize an image for display in the terminal, based on the aspect ratio
// (width/height) of the terminal characters and the maximum size. TODO:
// Make the size configurable.
fn resize(inimg: image::DynamicImage, x: u32, y: u32) -> image::DynamicImage {
    // This is the aspect ratio of each terminal character, equal to its width
    // divided by its height. This value is a good approximation for GNOME
    // Terminal on Linux but other platforms have different values.
    let aspect = 0.5;
    // Resize to fit a standard 80x24 terminal.
    let xmax: u32 = x;
    let ymax: u32 = y;
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
    // Pick the right rendering method based on what the user wants.
    if mode == "pound" {
        render_pound(img);
    } else if mode == "ascii" {
        render_ascii(img);
    } else if mode == "8color" {
        render_ansi(img);
    } else {
        println!("Invalid rendering mode {}. This is a programmer error.", mode);
        process::exit(1);
    }
}

// Display an image in the terminal by printing an array of spaces and pounds.
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

// Display an image using an ASCII art style.
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


// Display an image using ANSI color.
fn render_ansi(img: image::DynamicImage) {
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            // Get the red, green, and blue channels of the pixel and send them
            // to the quantize function.
            let channels = img.get_pixel(x, y).data;
            let best_color = quantize_ansi(channels[0], channels[1], channels[2]);
            match best_color {
                0 => print!("{} ", color::Bg(color::Black)),
                1 => print!("{} ", color::Bg(color::Red)),
                2 => print!("{} ", color::Bg(color::Green)),
                3 => print!("{} ", color::Bg(color::Yellow)),
                4 => print!("{} ", color::Bg(color::Blue)),
                5 => print!("{} ", color::Bg(color::Magenta)),
                6 => print!("{} ", color::Bg(color::Cyan)),
                7 => print!("{} ", color::Bg(color::White)),
                // We _should_ only get colors 0 through 7 but a little
                // defensive programming never hurts.
                _ => print!("{} ", color::Bg(color::White)),
            }
        }
        // Reset colors at the end of each line. If we don't do this, the
        // color of the rightmost pixel in each line is extended to the right
        // edge of the screen.
        println!("{}", color::Bg(color::Reset));
    }
}

fn quantize_ansi(red: u8, green: u8, blue:u8) -> u8 {
// TODO: This palette is a quick-and-dirty approximation.
// This is the palette that stores the color values of the eight basic terminal
// colors. The elements in the inner arrays are red, green, blue. Note that
// the actual colors vary depending on the terminal.
let palette: [[u8; 3]; 8] = [[0, 0, 0], [255, 0, 0], [0, 255, 0], [255, 255, 0], [0, 0, 255], [255, 0, 255], [0, 255, 255], [255, 255, 255]];
    // We set min_error initially to 442 because that's higher than the highest
    // possible value it could have: the error between the colors (0, 0, 0) and
    // (255, 255, 255) is only 441.67.
    let mut min_error = 442.0;
    let mut min_error_color: u8 = 0;
    for i in 0..palette.len() {
        // Calculate the Euclidean distance from the palette color to the
        // actual color of the pixel.
        let error = f32::sqrt( ((red as i32 - palette[i][0] as i32).pow(2) + (green as i32 - palette[i][1] as i32).pow(2) + (blue as i32 - palette[i][2] as i32).pow(2)) as f32 );
        if error < min_error {
            min_error = error;
            min_error_color = i as u8;
        }
    }
    return min_error_color;
}
