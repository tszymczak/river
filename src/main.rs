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
use std::f32;
extern crate image;
use image::{GenericImage, FilterType};
extern crate termion;
use termion::color;
extern crate clap;
use clap::{App, Arg};
extern crate exoquant;
use exoquant::*;

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
            .possible_values(&["pound", "ascii", "ascii-simple", "8colors", "16colors", "256colors", "truecolor"]))
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
    // ascii. Invalid values are handled by the library that handles arguments.
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
    } else if mode == "ascii-simple" {
        render_ascii_simple(img);
    } else if mode == "8colors" {
        render_8colors(img);
    } else if mode == "16colors" {
        render_16colors(img);
    } else if mode == "256colors" {
        render_256colors(img);
    } else if mode == "truecolor" {
        render_truecolor(img);
    } else {
        println!("Invalid rendering mode {}. This is a programmer error.", mode);
        process::exit(1);
    }
}

// Display an image in the terminal by printing an array of spaces and pounds.
fn render_pound(img: image::DynamicImage) {
    let palette = vec![
        Color { r: 0, g: 0, b: 0, a: 255 },
        Color { r: 255, g: 255, b: 255, a: 255 },
    ];

    let (width, height) = img.dimensions();

    // Convert image into a format exoquant can understand.
    let exo_img = image_to_exoquant(img);

    // Use exoquant to quantize the image according to our palette.
    let colorspace = SimpleColorSpace::default();
    let ditherer = ditherer::None;
    let remapper = Remapper::new(&palette, &colorspace, &ditherer);
    let quant_img = remapper.remap(&exo_img, width as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel_color = quant_img[(width*y + x) as usize];
            match pixel_color {
                0 => print!("#"),
                1 => print!(" "),
                _ => print!(" "),
            }
        }
        println!();
    }
}

// Display an image using an ASCII art style.
fn render_ascii(img: image::DynamicImage) {
    let palette = vec![
        Color { r: 0, g: 0, b: 0, a: 255 },
        Color { r: 32, g: 32, b: 32, a: 255 },
        Color { r: 64, g: 64, b: 64, a: 255 },
        Color { r: 96, g: 96, b: 96, a: 255 },
        Color { r: 128, g: 128, b: 128, a: 255 },
        Color { r: 160, g: 160, b: 160, a: 255 },
        Color { r: 192, g: 192, b: 192, a: 255 },
        Color { r: 224, g: 224, b: 224, a: 255 },
        Color { r: 255, g: 255, b: 255, a: 255 },
     ];

    let (width, height) = img.dimensions();

    // Convert image into a format exoquant can understand.
    let exo_img = image_to_exoquant(img);

    // Use exoquant to quantize the image according to our palette.
    let colorspace = SimpleColorSpace::default();
    let ditherer = ditherer::None;
    let remapper = Remapper::new(&palette, &colorspace, &ditherer);
    let quant_img = remapper.remap(&exo_img, width as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel_color = quant_img[(width*y + x) as usize];
            match pixel_color {
                0 => print!("W"),
                1 => print!("O"),
                2 => print!("L"),
                3 => print!(";"),
                4 => print!(":"),
                5 => print!("'"),
                6 => print!("-"),
                7 => print!(" "),
                _ => print!(" "),
            }
        }
        println!();
    }
}


// Display an image using an ASCII art style that's somewhat simpler than the
// regular one.
fn render_ascii_simple(img: image::DynamicImage) {
    let palette = vec![
        Color { r: 0, g: 0, b: 0, a: 255 },
        Color { r: 64, g: 64, b: 64, a: 255 },
        Color { r: 128, g: 128, b: 128, a: 255 },
        Color { r: 160, g: 160, b: 160, a: 255 },
        Color { r: 255, g: 255, b: 255, a: 255 },
     ];

    let (width, height) = img.dimensions();

    // Convert image into a format exoquant can understand.
    let exo_img = image_to_exoquant(img);

    // Use exoquant to quantize the image according to our palette.
    let colorspace = SimpleColorSpace::default();
    let ditherer = ditherer::None;
    let remapper = Remapper::new(&palette, &colorspace, &ditherer);
    let quant_img = remapper.remap(&exo_img, width as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel_color = quant_img[(width*y + x) as usize];
            match pixel_color {
                0 => print!("W"),
                1 => print!("O"),
                2 => print!("o"),
                3 => print!(":"),
                4 => print!(" "),
                _ => print!(" "),
            }
        }
        println!();
    }
}

// Display an image using ANSI color.
fn render_8colors(img: image::DynamicImage) {
    // This array is the palette of color values for the eight basic terminal
    // colors. In terms of data types, it's an array of exoquant::Color
    // structs. These values the values used in xterm (According to
    // https://jonasjacek.github.io/colors/ ) but are a reasonable
    // approximation for terminals in general.
    let palette = vec![
        Color { r: 0, g: 0, b: 0, a: 255 },
        Color { r: 128, g: 0, b: 0, a: 255 },
        Color { r: 0, g: 128, b: 0, a: 255 },
        Color { r: 128, g: 128, b: 0, a: 255 },
        Color { r: 0, g: 0, b: 128, a: 255 },
        Color { r: 128, g: 0, b: 128, a: 255 },
        Color { r: 0, g: 128, b: 128, a: 255 },
        Color { r: 192, g: 192, b: 192, a: 255 },
    ];

    let (width, height) = img.dimensions();

    // Convert image into a format exoquant can understand.
    let img_vec = image_to_exoquant(img);

    // Use exoquant to quantize the image according to our palette.
    let colorspace = SimpleColorSpace::default();
    // TODO: Make it possible to choose the dithering, choose a reasonable
    // default.
    let ditherer = ditherer::None;
    let remapper = Remapper::new(&palette, &colorspace, &ditherer);
    let indexed_data = remapper.remap(&img_vec, width as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel_color = indexed_data[(width*y + x) as usize];
            match pixel_color {
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

// Display an image using ANSI color.
fn render_16colors(img: image::DynamicImage) {
    // This array is the palette of color values for the 16 basic terminal
    // colors. In terms of data types, it's an array of exoquant::Color
    // structs. These values the values used in xterm (According to
    // https://jonasjacek.github.io/colors/ ) but are a reasonable
    // approximation for terminals in general.
    let palette = vec![
        Color { r: 0, g: 0, b: 0, a: 255 },
        Color { r: 128, g: 0, b: 0, a: 255 },
        Color { r: 0, g: 128, b: 0, a: 255 },
        Color { r: 128, g: 128, b: 0, a: 255 },
        Color { r: 0, g: 0, b: 128, a: 255 },
        Color { r: 128, g: 0, b: 128, a: 255 },
        Color { r: 0, g: 128, b: 128, a: 255 },
        Color { r: 192, g: 192, b: 192, a: 255 },
        Color { r: 128, g: 128, b: 128, a: 255 },
        Color { r: 255, g: 0, b: 0, a: 255 },
        Color { r: 0, g: 255, b: 0, a: 255 },
        Color { r: 255, g: 255, b: 0, a: 255 },
        Color { r: 0, g: 0, b: 255, a: 255 },
        Color { r: 255, g: 0, b: 255, a: 255 },
        Color { r: 0, g: 255, b: 255, a: 255 },
        Color { r: 255, g: 255, b: 255, a: 255 },
    ];

    let (width, height) = img.dimensions();

    // Convert image into a format exoquant can understand.
    let img_vec = image_to_exoquant(img);

    // Use exoquant to quantize the image according to our palette.
    let colorspace = SimpleColorSpace::default();
    let ditherer = ditherer::None;
    let remapper = Remapper::new(&palette, &colorspace, &ditherer);
    let indexed_data = remapper.remap(&img_vec, width as usize);


    for y in 0..height {
        for x in 0..width {
            let pixel_color = indexed_data[(width*y + x) as usize];
            match pixel_color {
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

// Display images using 256 colors. Note that not all terminals can do this.
fn render_256colors(img: image::DynamicImage) {
    let palette = generate_256colors_palette();

    let (width, height) = img.dimensions();

    // Convert image into a format exoquant can understand.
    let img_vec = image_to_exoquant(img);

    // Use exoquant to quantize the image according to our palette.
    let colorspace = SimpleColorSpace::default();
    let ditherer = ditherer::None;
    let remapper = Remapper::new(&palette, &colorspace, &ditherer);
    let indexed_data = remapper.remap(&img_vec, width as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel_color = indexed_data[(width*y + x) as usize];
            print!("{} ", color::Bg(color::AnsiValue(pixel_color)));
        }
        // Reset colors at the end of each line. If we don't do this, the
        // color of the rightmost pixel in each line is extended to the right
        // edge of the screen.
        println!("{}", color::Bg(color::Reset));
    }
}

// Generate the palette of colors used for the 256 color mode. We could
// hardcode this but it also makes sense to generate it because the palette is
// large but fairly regular.
fn generate_256colors_palette() -> Vec<Color> {
    // Add the first 16 colors. These are the same as the 16 color mode.
    let mut palette = vec![
        Color { r: 0, g: 0, b: 0, a: 255 },
        Color { r: 128, g: 0, b: 0, a: 255 },
        Color { r: 0, g: 128, b: 0, a: 255 },
        Color { r: 128, g: 128, b: 0, a: 255 },
        Color { r: 0, g: 0, b: 128, a: 255 },
        Color { r: 128, g: 0, b: 128, a: 255 },
        Color { r: 0, g: 128, b: 128, a: 255 },
        Color { r: 192, g: 192, b: 192, a: 255 },
        Color { r: 128, g: 128, b: 128, a: 255 },
        Color { r: 255, g: 0, b: 0, a: 255 },
        Color { r: 0, g: 255, b: 0, a: 255 },
        Color { r: 255, g: 255, b: 0, a: 255 },
        Color { r: 0, g: 0, b: 255, a: 255 },
        Color { r: 255, g: 0, b: 255, a: 255 },
        Color { r: 0, g: 255, b: 255, a: 255 },
        Color { r: 255, g: 255, b: 255, a: 255 },
    ];
    
    // Generate the next 216 colors, which are the RGB colors. These colors
    // have a regular structure: there are 6 levels for each color channel, and
    // we use every possible combination, so we get 6^3=216 colors.
    // These are the values that each channel uses.
    let channel_vals = vec![ 0, 95, 135, 175, 215, 255 ];
    // Iterate through every possibility and add it.
    for r in 0..channel_vals.len() {
        for g in 0..channel_vals.len() {
            for b in 0..channel_vals.len() {
                palette.push(Color{ r: channel_vals[r], g: channel_vals[g], b: channel_vals[b], a: 255 });
            }
        }
    }
    
    // Finally, add the 24 grayscale colors. Each shade of gray has all three
    // color channels set to the same value, and for each color (starting at
    // zero) this value is 10*i + 8.
    for i in 0..24 {
        let x = 10*i + 8;
        palette.push(Color{ r: x, g: x, b: x, a: 255 });
    }
    
    return palette;
}

// Render an image in truecolor, i.e. 24-bit color.
fn render_truecolor(img: image::DynamicImage) {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            // Get the rgb values of the pixel.
            let pixel_channels = img.get_pixel(x, y).data;
            let red = pixel_channels[0];
            let green = pixel_channels[1];
            let blue = pixel_channels[2];
            print!("{} ", color::Bg(color::Rgb(red, green, blue)));
        }
        // Reset colors at the end of each line. If we don't do this, the
        // color of the rightmost pixel in each line is extended to the right
        // edge of the screen.
        println!("{}", color::Bg(color::Reset));        
    }
}

// Convert an image from the image libary's format into the format exoquant
// uses.
fn image_to_exoquant(input: image::DynamicImage) -> Vec<Color> {
    let (width, height) = input.dimensions();
    let mut img_vec: Vec<Color> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let channels = input.get_pixel(x, y).data;
            let new_color: Color = Color { r: channels[0], g: channels[1], b: channels[2], a: channels[3] };
            img_vec.push(new_color);
        }
    }

    return img_vec;
}

// Debug code: Print the color values of the palette.
fn print_palette(palette: &Vec<Color>) {
    for i in 0..palette.len() {
        let color = palette[i];
        println!("{} {} {} {}", color.r, color.g, color.b, color.a);
    }
}
