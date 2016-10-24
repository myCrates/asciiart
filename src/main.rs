// Main function handling the main phases:
//   - init phase (downloading asciiarts).
//   - main phase (select the asciiart to output

extern crate rand; // https://crates.io/crates/rand

use rand::{Rng};         // The generic trait all random generators support.
use rand::os::{OsRng};   // Specific implementation of above for strong crypto.

use std::fs::File;
use std::io::{Read, BufReader};
use std::path::PathBuf;

fn get_art() -> String {
    let mut output = String::new();
    let rp = PathBuf::from("/var/lib/assets/asciiartfarts.txt");
    let f = File::open(rp.to_str().unwrap()).expect("couldn't open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut output).expect("Unable to read string");
    return output;
}

fn main() {
    let art = get_art();
    // OsRng is a type of `Rng` that wraps /dev/urandom, getrandom(), etc.
    let mut r = OsRng::new().unwrap();

    // Random bytes.
    let mut my_secure_bytes = vec![0u8; 1500];
    r.fill_bytes(&mut my_secure_bytes);

    // Primitive types and short arrays.
    let my_secure_int: usize = r.gen_range(0, 5372);

    // println!("First few bytes = {:?}; random int = {:?}",
    //     &my_secure_bytes[..5], my_secure_int);
    println!("{}", art.split("\n%\n").nth(my_secure_int).unwrap());
    // println!("Response: {}", res.status);
    // println!("Headers:\n{}", res.headers);
    // println!("everything went fine");
}
