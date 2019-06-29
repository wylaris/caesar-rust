use std::fs;

use clap::{App, Arg, crate_version};

fn main() {
    let matches = build_app().get_matches();
    let file = matches.value_of("file").unwrap();
    let contents = fs::read_to_string(file).expect("Unable to read file");
    println!("File contents: \n\n{}", contents);
}


fn build_app() -> App<'static, 'static> {
    App::new("Julis Caesar Cipher - Rust")
        .version(crate_version!())
        .author("Peyton Schmidt <pcs7709@rit.edu>")
        .about("Encodes/Decodes messages")
        .args(&[
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("File to decode")
                .required(true),
            Arg::with_name("shift")
                .short("s")
                .long("shift")
                .value_name("SHIFT VALUE")
                .help("Value to shift")
                .required(true),
        ])
}
