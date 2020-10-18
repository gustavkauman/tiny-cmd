extern crate clap;
extern crate reqwest;

use clap::{Arg, App};
use reqwest::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("tiny")
        .version("0.1.0")
        .author("Gustav Utke Kauman <gkauman@eadu.dk>")
        .about("Command line tool for creating tiny URLs using the tinyurl.com API")
        .arg(Arg::with_name("Long URL")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("URL to shorten"))
        .get_matches();

    let url = matches.values_of("Long URL").unwrap().next().unwrap();

    println!("{}", get_short_url(url));

    Ok(())
}

fn get_short_url(long_url: &str) -> String {

    let base_api_url = "http://tinyurl.com/create.php";

    let mut body = HashMap::new(); 
    body.insert("url", long_url);

    let client = reqwest::blocking::Client::new();
    let res = client.post(base_api_url)
        .json(&body)
        .send()
        .unwrap();
    
    res.text().unwrap()
}
