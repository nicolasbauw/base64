use std::{io, str};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.pop();
    let a = input.as_bytes();

    println!("{:?}", a);
    let mut octal = String::new();

    let mut i = 0;
    println!("Length of string to encode : {}", a.len());

    // The bytes to process without padding, generating a full sextets table
    let bytestoprocess = match a.len() % 3 {
        0 => a.len(),
        _ => a.len() - (a.len() - a.len() % 3),
    };

    // Creating octal output from bytes converted to sextets (3 * 8 = 24 bytes)
    while i < bytestoprocess {
        octal.push_str(format!("{:o}", u32::from_be_bytes([0, a[i], a[i + 1], a[i + 2]])).as_str());
        i += 3;
    }
    println!("{}", octal);

    // Converting octal output to a decimal index vector
    let sextets = octal
        .as_bytes()
        .chunks(2)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .iter()
        .map(|s| u8::from_str_radix(s, 8))
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();

    println!("{:?}", sextets);
}
