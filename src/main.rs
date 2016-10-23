// Main function handling the main phases:
//   - init phase (downloading asciiarts).
//   - main phase (select the asciiart to output

extern crate hyper;
extern crate kuchiki;

use std::io;

use hyper::client::Client;
use kuchiki::NodeRef;
use kuchiki::traits::ParserExt;

fn main() {
    let client = Client::new();

    let res = client.get("http://www.asciiartfarts.com/19990625.html")
        .send().unwrap();
    let document = kuchiki::parse_html().from_http(res).unwrap();

    for node in document.select("pre") {
        for child in node {
            println!("{}", child.text_contents());
        }
    }

    // println!("Response: {}", res.status);
    // println!("Headers:\n{}", res.headers);
    println!("everything went fine");
}
