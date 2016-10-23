// Main function handling the main phases:
//   - init phase (downloading asciiarts).
//   - main phase (select the asciiart to output

extern crate hyper;
extern crate kuchiki;

use hyper::client::Client;
use kuchiki::traits::ParserExt;

fn get_art(url: String) -> String {
    let client = Client::new();

    let res = client.get(&url)
        .send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    let document = kuchiki::parse_html().from_http(res).unwrap();

    return document.select("pre").unwrap().last().unwrap().text_contents();
}

fn main() {
    let url = "http://www.asciiartfarts.com/19990625.html";
    let art = get_art(url.to_string());
    println!("{}", art);
    // println!("Response: {}", res.status);
    // println!("Headers:\n{}", res.headers);
    println!("everything went fine");
}
