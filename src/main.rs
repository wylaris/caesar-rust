use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{App, Arg, crate_version};

fn main() {
    let matches = build_app().get_matches();
    let file_arg = matches.value_of("file").unwrap();
    let file = open_file(&file_arg).unwrap();
    for line in BufReader::new(file).lines() {
        let encoded = handle_line(line.unwrap()).unwrap();
        println!("{}", encoded);
    }
}

fn open_file(path: &str) -> Result<File, std::io::Error> {
    let file = File::open(path)?;
    Ok(file)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn run(filename: String, _0: i8) -> String {
        let mut output = String::new();
        let file = open_file(&filename).unwrap();
        for line in BufReader::new(file).lines() {
            let encoded = handle_line(line.unwrap()).unwrap();
            output.push_str(&encoded);
            output.push_str("\n");
        }
        output
    }

    fn original_text(filename: &String) -> String {
        fs::read_to_string(filename).expect("Unable to read contents")
    }

    #[test]
    fn test_unchanged_single() {
        let test_file = "tests/simple_test.txt".to_string();
        let expected = original_text(&test_file);
        let outputted = run(test_file, 0);
        assert_eq!(expected, outputted);
    }

    #[test]
    fn test_unchanged_multiple() {
        let test_file = "tests/simple_multilined.txt".to_string();
        let expected = original_text(&test_file);
        let outputted = run(test_file, 0);
        assert_eq!(expected, outputted);
    }
}
