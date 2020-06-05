use std::{io, str};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.pop();
    let a = input.as_bytes();

    let mut octal = String::new();
    let mut i = 0;

    // The bytes to process without padding, generating a full sextets table
    let blockstoprocess = match a.len() % 3 {
        0 => a.len(),
        _ => a.len() - a.len() % 3,
    };
    let padding = match a.len() % 3 {
        0 => 0,
        _ => 3 - (a.len() - blockstoprocess)
    };

    println!("{:?}", a);
    println!("Length of string to encode : {}", a.len());
    println!("24 bits blocks to process : {}", blockstoprocess);
    println!("Padding : {}", padding);

    // Creating octal output from bytes converted to sextets (3 * 8 = 24 bytes)
    while i < blockstoprocess {
        octal.push_str(format!("{:o}", u32::from_be_bytes([0, a[i], a[i + 1], a[i + 2]])).as_str());
        i += 3;
    }
    println!("{}", octal);

    // Converting octal output to a decimal index vector
    let sextets = octal
        .as_bytes()
        .chunks(2)
        .map(|s| {
            u8::from_str_radix(str::from_utf8(s).unwrap(), 8).unwrap()
        })
        .collect::<Vec<u8>>();

    println!("{:?}", sextets);
}
