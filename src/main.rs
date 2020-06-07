use std::io;
use lib_base64::Base64;

fn main() {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() == true { return };

    // removes line feed
    input.pop();

    println!("{}", input.encode());
}
