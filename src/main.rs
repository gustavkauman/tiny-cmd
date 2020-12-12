use clap::{Arg, App};
use reqwest::blocking::Client;
use std::collections::HashMap;

fn main() {
    let matches = App::new("tiny")
        .version("1.0.0")
        .author("Gustav Utke Kauman <gkauman@eadu.dk>")
        .about("Command line tool for creating tiny URLs using the tinyurl.com API")
        .arg(Arg::with_name("Long URL")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("URL to shorten"))
        .get_matches();

    let value = match matches.values_of("Long URL") {
        Some(mut value) => { value.next() }
        None => {
            println!("An error occured while processing the parameters given. Please make sure, they are correct");
            std::process::exit(1);
        }
    };

    let url = match value {
        Some(url) => { url }
        None => {
            println!("An error occured while processing the parameters given. Please make sure, they are correct");
            std::process::exit(1);
        }
    };

    println!("Shortening your URL \"{}\"...", url);

    let short_url = match get_short_url(url) {
        Some(url) => { url }
        None => {
            println!("An error occured while processing your request. Please make sure, the url is valid");
            std::process::exit(1);
        }
    };

    println!("Your shortened URL is: {}", short_url);
}

fn get_short_url(long_url: &str) -> Option<String> {

    let base_api_url = "http://tinyurl.com/create.php";

    let mut body = HashMap::new(); 
    body.insert("url", long_url);

    let client = Client::new();
    let result = client.post(base_api_url)
        .json(&body)
        .send();


    let response = match result {
        Ok(response) => { response }
        Err(_) => {
            println!("An error occured while processing your request. Please make sure, the url is valid");
            std::process::exit(1);
        }
    };

    if response.status().is_success() {
        
        match response.text() {
            Ok(val) => Some(val),
            Err(_) => None
        }

    } else {
        None
    }
}
