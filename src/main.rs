use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{App, Arg, crate_version};

fn main() {
    let matches = build_app().get_matches();
    let file_arg = matches.value_of("file").unwrap();
    let file = File::open(&file_arg).unwrap();
    for line in BufReader::new(file).lines() {
        let encoded = handle_line(line.unwrap()).unwrap();
        println!("{}", encoded);
    }
}


fn handle_line(line: String) -> Result<String,()> {
    let mut output = String::new();
    for char in line.chars() {
        output.push(char);
    }
    Ok(output)
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
