use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{App, Arg, crate_version};

/// Main function that creates the app and runs the program
fn main() {
    let matches = build_app().get_matches();
    let file_arg = matches.value_of("file").unwrap();
    let file = open_file(&file_arg).unwrap();
    for line in BufReader::new(file).lines() {
        let encoded = handle_line(line.unwrap()).unwrap();
        println!("{}", encoded);
    }
}


/// Attempts to open a file
fn open_file(path: &str) -> Result<File, std::io::Error> {
    let file = File::open(path)?;
    Ok(file)
}


/// Takes a single line from a file an performs the shift
fn handle_line(line: String) -> Result<String,()> {
    let mut output = String::new();
    for char in line.chars() {
        output.push(char);
    }
    Ok(output)
}


/// Creates the app and returns the values from the command line
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
/// Test suite
mod tests {
    use super::*;
    use std::fs;


    /// Simulates the work done by the main function
    ///
    /// Instead of printing the results they are returned as
    /// a built up string: output
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


    /// Gets the original text from a file and returns it
    ///
    /// Used to test when a shift of 0 is used
    fn original_text(filename: &String) -> String {
        fs::read_to_string(filename).expect("Unable to read contents")
    }


    #[test]
    /// Tests a single lined file with a shift of 0
    fn test_unchanged_single() {
        let test_file = "tests/simple_test.txt".to_string();
        let expected = original_text(&test_file);
        let outputted = run(test_file, 0);
        assert_eq!(expected, outputted);
    }

    #[test]
    /// Tests a multilined file with a shift of 0
    fn test_unchanged_multiple() {
        let test_file = "tests/simple_multilined.txt".to_string();
        let expected = original_text(&test_file);
        let outputted = run(test_file, 0);
        assert_eq!(expected, outputted);
    }
}
