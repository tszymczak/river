# river
River is a program that prints out images in the terminal using ASCII art
or colored blocks.

Written by Thomas Szymczak, @tszymczak on GitHub.

## Building and running
This quick guide assumes you already have Rust installed on your system.

First download and compile the code:
```
git clone https://github.com/tszymczak/river
cd river/
cargo build --release
```
Building with ```--release``` takes longer to compile but the built program is faster. If you omit the ```--release``` option when compiling, omit it when running as well.

Next run it with:
```
cargo run --release ./image.jpg
```

For the help text run
```
cargo run --release -- -h
```

Or run the executable directly. The exact command might vary depending on your platform but for me it is like this:
```
./target/release/river -h
```
