//! An alternative and interactive base64 (with padding) string encoding/decoding utility.
//!
//! You can use it by two ways:
//! - Interactive mode : you run it, type your text, and press enter
//! - Non-interactive mode : you echo your text and pipe it to base64-lt.
//!
//! The -d commutator is for decoding. Tested on MacOS / Linux / Windows.
//!
//! Examples:
//! ```text
//! base64-lt   
//! Test
//! VGVzdA==
//! ````
//!
//! ```text
//! base64-lt -d
//! VGVzdA==
//! Test
//! ````
//!
//! ```text
//! echo "VGVzdA==" | base64-lt -d
//! Test
//! ```

use lib_base64::Base64;
use std::{env, io};

static VERSION: &str = env!("CARGO_PKG_VERSION");
static HELP: &str = "A base64 (with padding) string encoding/decoding utility.

USAGE:
    Interactive mode     : base64-lt [FLAGS]
    Non-interactive mode : echo \"string\" | base64-lt [FLAGS]

FLAGS:
    -d               decodes a base64-encoded string
    -h, --help       Prints help information
    -V, --version    Prints version information
    
EXAMPLE:
    echo \"VGVzdA==\" | base64-lt -d";

fn main() -> Result<(), lib_base64::Base64Error> {
    let mut args = env::args();
    let mut input = String::new();

    let decode = match args.nth(1) {
        None => false,
        Some(a) => match a.as_ref() {
            "-d" => true,
            "-h" | "--help" => {
                println!("base64-lt {}", VERSION);
                println!("{}", HELP);
                return Ok(());
            }
            "-V" | "--version" => {
                println!("{}", VERSION);
                return Ok(());
            }
            _ => {
                println!("Invalid argument");
                return Ok(());
            }
        },
    };
    if io::stdin().read_line(&mut input).is_err() == true {
        println!("Can't read stdin");
        return Ok(());
    };

    // removes line feed
    input.pop();

    // Windows : removes carriage return
    #[cfg(windows)]
    input.pop();

    match decode {
        false => println!("{}", input.encode()),
        true => println!("{}", input.decode()?),
    }
    Ok(())
}
