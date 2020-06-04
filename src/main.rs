use std::{io, str};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.pop();
    let a = input.as_bytes();

    println!("{:?}", a);
    let octal = format!("{:o}", u32::from_be_bytes([0, a[0], a[1], a[2]]));
    println!("{}", octal);

    let sextets = octal.as_bytes()
    .chunks(2)
    .map(str::from_utf8)
    .collect::<Result<Vec<&str>, _>>()
    .unwrap();

    println!("{:?}", sextets);
}
