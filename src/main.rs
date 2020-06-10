use std::io;
use lib_base64::Base64;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "An alternative version of base64")]
struct Opt {
    #[structopt(short = "d")]
    // We have to encore or decode ?
    decode: bool
}

fn main() {
    let opt = Opt::from_args();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() == true { return };

    // removes line feed
    input.pop();

    // Windows : removes carriage return
    #[cfg(windows)]
    input.pop();

    match opt.decode {
        false => println!("{}", input.encode()),
        true => println!("{}", input.decode())
    }
}
