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
    if res.status == hyper::NotFound {
        return "nothing".to_string();
    }
    let document = kuchiki::parse_html().from_http(res).unwrap();

    return document.select("pre").unwrap().last().unwrap().text_contents();
}

fn main() {
    let url = "http://www.asciiartfarts.com/";
    for year in 1999..2015 {
        for month in 1..13 {
            for day in 1..32 {
                let art_url = url.to_string() + &year.to_string() + &format!("{:02}", month) + &format!("{:02}", day) + &".html".to_string();
                println!("{}", art_url);
                let art = get_art(art_url);
                println!("{}", art);
            }
        }
    }
    // println!("Response: {}", res.status);
    // println!("Headers:\n{}", res.headers);
    println!("everything went fine");
}
