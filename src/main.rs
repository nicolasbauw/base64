//! An alternative and interactive base64 (with padding) string encoding/decoding utility.
//! 
//! You can use it by two ways:
//! - You run it, type your text, and press enter
//! - You echo your text and pipe it to base64-lt.
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
use std::io;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "A base64 (with padding) string encoding/decoding utility.")]
struct Opt {
    #[structopt(short = "d")]
    /// decodes a base64-encoded string
    decode: bool,
}

fn main() -> Result<(), lib_base64::Base64Error> {
    let opt = Opt::from_args();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() == true {
        println!("Can't read stdin");
        return Ok(());
    };

    // removes line feed
    input.pop();

    // Windows : removes carriage return
    #[cfg(windows)]
    input.pop();

    match opt.decode {
        false => println!("{}", input.encode()),
        true => println!("{}", input.decode()?),
    }
    Ok(())
}
