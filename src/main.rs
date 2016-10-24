// Main function handling the main phases:
//   - init phase (downloading asciiarts).
//   - main phase (select the asciiart to output

extern crate hyper;

use std::process::Command;
use std::fs::File;
use std::io::{Read, Write, BufWriter};

use hyper::client::Client;

fn get_art(url: String) -> String {
    let client = Client::new();

    let mut output = String::new();
    let mut res = client.get(&url).send().unwrap();

    res.read_to_string(&mut output).expect("no response body");
    let pwd = Command::new("sh").arg("-c").arg("pwd").output().expect("Failed to pwd");
    println!("{}", String::from_utf8_lossy(&pwd.stdout));
    let f = File::create("assets/asciiartfarts.com").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(output.as_bytes()).expect("Unable to write data");
    return output;
}

fn main() {
    let url = "http://www.asciiartfarts.com/fortune.txt";
    let art = get_art(url.to_string());
    println!("{}", art);
    // println!("Response: {}", res.status);
    // println!("Headers:\n{}", res.headers);
    println!("everything went fine");
}
