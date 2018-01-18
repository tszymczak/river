// A Rust program that prints images to the terminal. A fun, if useless,
// project that taught me to code in Rust.

// Copyright 2018 Thomas Szymczak
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::path::Path;
use std::process;
use std::{f32, i32};
extern crate image;
use image::{GenericImage, Pixel, FilterType};
extern crate termion;
use termion::color;
extern crate clap;
use clap::{App, Arg};

fn main() {
    // Parse command line input.
    let matches = App::new("River")
        .version("0.3.0")
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
            .possible_values(&["pound", "ascii", "asciisimple", "8color", "16color"]))
        .arg(Arg::with_name("height")
            .help("Manually set the height of the terminal in columns.")
            .short("y")
            .takes_value(true))
        .arg(Arg::with_name("width")
            .help("Manually set the width of the terminal in rows.")
            .short("x")
            .takes_value(true))
        .arg(Arg::with_name("ratio")
            .help("Set the aspect ratio (width divided by height) of the terminal's characters.")
            .short("r")
            .takes_value(true))
        .get_matches();

    // Get the input file name. Crash if not specified.
    let infile_name;
    if matches.is_present("INPUT") {
        infile_name = matches.value_of("INPUT").unwrap();
    } else {
        println!("No input file name supplied!");
        process::exit(1);
    }

    // Get the dimensions of the terminal window. The code is rather lenghty
    // so it has its own method.
    let (x, y): (u32, u32) = choose_dimensions(&matches);

    // Handle mode inputs. If the user doesn't specify a mode, default to
    // ascii. Invalid values are handles by the library that handles arguments.
    let mode = matches.value_of("mode").unwrap_or("ascii");

    // Get the aspect ratio. If not specified by the user, 0.5 is a reasonable
    // default.
    let ratio: f32;
    let default_ratio: f32 = 0.5;
    if matches.is_present("ratio") {
        match matches.value_of("ratio").unwrap().parse::<f32>() {
            Ok(n) => ratio = n,
            Err(_) => {
                eprintln!("Invalid value `{}' for aspect ratio, defaulting to {}.", matches.value_of("ratio").unwrap(), default_ratio);
                ratio = default_ratio;
            },
        }
    } else {
        ratio = default_ratio;
    }

    // Open the input image file and resize it.
    let inimg = image::open(&Path::new(&infile_name)).ok().expect("Opening image failed");
    let img = resize(inimg, x, y, ratio);

    // Render the image to the terminal.
    render(img, mode);
}

// Determine the dimensions to print the image with, based on the arguments
// given, the size of the terimanl, and the default size if all else fails.
fn choose_dimensions(matches: &clap::ArgMatches) -> (u32, u32) {
    // Somewhat messy but does what I want. Definitely some technical debt in
    // here.
    let (default_x, default_y): (u32, u32) = (80, 24);
    let x: u32;
    let y: u32;
    let det_x: u32;
    let det_y: u32;

    let term_result = termion::terminal_size();
    let ok;
    if term_result.is_ok() {
        let (a, b) = term_result.unwrap();
        det_x = a as u32;
        det_y = b as u32;
        ok = true;
    } else {
        det_x = 0;
        det_y = 0;
        ok = false;
    }

    if matches.is_present("width") {
        match matches.value_of("width").unwrap().parse::<u32>() {
            Ok(n) => x = n,
            Err(_) => {
                eprintln!("Invalid value `{}' for terminal width, attempting to autodetect.", matches.value_of("width").unwrap());
                if ok {
                    x = det_x;
                } else {
                    eprintln!("Warning: Can't autodetect terminal width, assuming {}.", default_x);
                    x = default_x;
                }
            },
        }
    } else {
        if ok {
            x = det_x;
        } else {
            eprintln!("Warning: Can't autodetect terminal width, assuming {}.", default_x);
            x = default_x;
        }
    }

    if matches.is_present("height") {
        match matches.value_of("height").unwrap().parse::<u32>() {
            Ok(n) => y = n,
            Err(_) => {
                eprintln!("Invalid value `{}' for terminal height, attempting to autodetect.", matches.value_of("height").unwrap());
                if ok {
                    y = det_y;
                } else {
                    eprintln!("Warning: Can't autodetect terminal height, assuming {}.", default_y);
                    y = default_y;
                }
            },
        }
    } else {
        if ok {
            y = det_y;
        } else {
            eprintln!("Warning: Can't autodetect terminal height, assuming {}.", default_y);
            y = default_y;
        }
    }

    return (x, y);
}

// Resize an image for display in the terminal, based on the aspect ratio
// (width/height) of the terminal characters and the maximum size.
fn resize(inimg: image::DynamicImage, x: u32, y: u32, aspect: f32) -> image::DynamicImage {
    // This is the aspect ratio of each terminal character, equal to its width
    // divided by its height. This value is a good approximation for GNOME
    // Terminal on Linux but other platforms have different values.
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
    } else if mode == "asciisimple" {
        render_asciisimple(img);
    } else if mode == "8color" {
        render_8color(img);
    } else if mode == "16color" {
        render_16color(img);
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
            if luma < 32 {
                print!("W");
            } else if luma < 64 {
                print!("O");
            } else if luma < 96 {
                print!("L");
            } else if luma < 128 {
                print!(";");
            } else if luma < 160 {
                print!(":");
            } else if luma < 192 {
                print!("'");
            } else if luma < 224 {
                print!("-");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}


// Display an image using an ASCII art style.
fn render_asciisimple(img: image::DynamicImage) {
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            // Get the brightness level of the pixel.
            let luma = img.get_pixel(x, y).to_luma().data[0];
            if luma < 32 {
                print!("W");
            } else if luma < 64 {
                print!("O");
            } else if luma < 128 {
                print!("o");
            } else if luma < 192 {
                print!(":");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Display an image using ANSI color.
fn render_8color(img: image::DynamicImage) {
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            // Get the red, green, and blue channels of the pixel and send them
            // to the quantize function.
            let channels = img.get_pixel(x, y).data;
            let best_color = quantize_8color(channels[0], channels[1], channels[2]);
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

fn quantize_8color(red: u8, green: u8, blue:u8) -> u8 {
// This array is the palette of color values for the eight basic terminal
// colors. The elements in the inner arrays are red, green, blue. The color
// values here are the values used in xterm (According to
// https://jonasjacek.github.io/colors/ ) but are a reasonable approximation
// for terminal in general.
let palette: [[u8; 3]; 8] = [[0, 0, 0], [128, 0, 0], [0, 128, 0], [128, 128, 0], [0, 0, 128], [128, 0, 128], [0, 128, 128], [192, 192, 192]];
    // We set min_error initially to 442 because that's higher than the highest
    // possible value it could have: the error between the colors (0, 0, 0) and
    // (255, 255, 255) is only 441.67.
    let mut best_error = 442.0;
    let mut best_color: u8 = 0;
    for i in 0..palette.len() {
        // Calculate the Euclidean distance from the palette color to the
        // actual color of the pixel.
        let error = f32::sqrt( ((red as i32 - palette[i][0] as i32).pow(2) + (green as i32 - palette[i][1] as i32).pow(2) + (blue as i32 - palette[i][2] as i32).pow(2)) as f32 );
        if error < best_error {
            best_error = error;
            best_color = i as u8;
        }
    }
    return best_color;
}

// Display an image using ANSI color.
fn render_16color(img: image::DynamicImage) {
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            // Get the red, green, and blue channels of the pixel and send them
            // to the quantize function.
            let channels = img.get_pixel(x, y).data;
            let best_color = quantize_16color(channels[0], channels[1], channels[2]);
            match best_color {
                0 => print!("{} ", color::Bg(color::Black)),
                1 => print!("{} ", color::Bg(color::Red)),
                2 => print!("{} ", color::Bg(color::Green)),
                3 => print!("{} ", color::Bg(color::Yellow)),
                4 => print!("{} ", color::Bg(color::Blue)),
                5 => print!("{} ", color::Bg(color::Magenta)),
                6 => print!("{} ", color::Bg(color::Cyan)),
                7 => print!("{} ", color::Bg(color::White)),
                8 => print!("{} ", color::Bg(color::LightBlack)),
                9 => print!("{} ", color::Bg(color::LightRed)),
                10 => print!("{} ", color::Bg(color::LightGreen)),
                11 => print!("{} ", color::Bg(color::LightYellow)),
                12 => print!("{} ", color::Bg(color::LightBlue)),
                13 => print!("{} ", color::Bg(color::LightMagenta)),
                14 => print!("{} ", color::Bg(color::LightCyan)),
                15 => print!("{} ", color::Bg(color::LightWhite)),
                // We _should_ only get colors 0 through 15 but a little
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

fn quantize_16color(red: u8, green: u8, blue:u8) -> u8 {
// This array is the palette of color values for the eight basic terminal
// colors. The elements in the inner arrays are red, green, blue. The color
// values here are the values used in xterm (According to
// https://jonasjacek.github.io/colors/ ) but are a reasonable approximation
// for terminal in general.
let palette: [[u8; 3]; 16] = [[0, 0, 0], [128, 0, 0], [0, 128, 0], [128, 128, 0], [0, 0, 128], [128, 0, 128], [0, 128, 128], [192, 192, 192], [128, 128, 128], [255, 0, 0], [0, 255, 0], [255, 255, 0], [0, 0, 255], [255, 0, 255], [0, 255, 255], [255, 255, 255]];
    // We set min_error initially to 442 because that's higher than the highest
    // possible value it could have: the error between the colors (0, 0, 0) and
    // (255, 255, 255) is only 441.67.
    let mut best_error = 442.0;
    let mut best_color: u8 = 0;
    for i in 0..palette.len() {
        // Calculate the Euclidean distance from the palette color to the
        // actual color of the pixel.
        let error = f32::sqrt( ((red as i32 - palette[i][0] as i32).pow(2) + (green as i32 - palette[i][1] as i32).pow(2) + (blue as i32 - palette[i][2] as i32).pow(2)) as f32 );
        if error < best_error {
            best_error = error;
            best_color = i as u8;
        }
    }
    return best_color;
}
