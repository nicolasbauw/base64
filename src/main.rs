use std::{io, str};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.pop();
    let a = input.as_bytes();

    let mut octal = String::new();
    let mut i = 0;

    // The number of full sextets to process
    let blockstoprocess = match a.len() % 3 {
        0 => a.len(),
        _ => a.len() - a.len() % 3,
    };
    let padding = match a.len() % 3 {
        0 => 0,
        _ => 3 - (a.len() - blockstoprocess),
    };

    // Creating octal output from bytes converted to sextets (3 * 8 bytes = 24 bits = four sextets)
    while i < blockstoprocess {
        octal.push_str(format!("{:o}", u32::from_be_bytes([0, a[i], a[i + 1], a[i + 2]])).as_str());
        i += 3;
    }

    match padding {
        1 => {
            octal.push_str(format!("{:o}", u32::from_be_bytes([0, a[i], a[i + 1], 0])).as_str());
        }
        2 => {
            octal.push_str(format!("{:o}", u32::from_be_bytes([0, a[i], 0, 0])).as_str());
        }
        _ => {}
    };

    // Converting octal output to a decimal index vector
    let sextets = octal
        .as_bytes()
        .chunks(2)
        .map(|s| u8::from_str_radix(str::from_utf8(s).unwrap(), 8).unwrap())
        .collect::<Vec<u8>>();

    #[cfg(debug_assertions)] {
        // For dev and debug
        println!("{:?}", a);
        println!("Length of string to encode : {}", a.len());
        println!("24 bits blocks to process : {}", blockstoprocess);
        println!("Padding : {}", padding);
        println!("{}", octal);
        println!("{:?}", sextets);
    }

    let table = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for i in 0..sextets.len() {
        print!("{}", &table[sextets[i] as usize..(sextets[i]+1) as usize]);
    }
    println!("");
}
