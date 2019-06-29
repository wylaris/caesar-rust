# Caesar Cipher - Rust #

## Description ##
This is a simple Caesar Cipher written in Rust for the purpose of showing new Rustaceans
about the glories of Rust and some practices I learned on the job.

I recommend running this application with Cargo as it makes the process easier on the user
and allows you to use the following commands

Arguments:
```
USAGE:
    caesar-rust --file <FILE> --shift <SHIFT VALUE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>            File to decode
    -s, --shift <SHIFT VALUE>    Value to shift

```

## Execution ##
Execution: `cargo run -- --file tests/simple_test.txt`

Testing: `cargo test`
