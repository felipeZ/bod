/*!
# Bod
Library to interact with online auction sites

*/

extern crate reqwest;
extern crate serde_json;

mod bidder;
mod communicator;

pub use bidder::Bidder;
use reqwest::Url;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

pub fn perform_transaction(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(url).expect("Invalid URL");
    let response = reqwest::blocking::get(url.as_str())?;
    let body = response.text()?;
    let mut file = File::create("reply.json")?;
    file.write(body.as_bytes())?;
    Ok(())
}

pub fn deserialize_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("reply.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&contents)?;
    Ok(())
}
