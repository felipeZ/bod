extern crate bod;
extern crate clap;

use bod::{perform_transaction, deserialize_json};

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Bod")
        .version("0.0.1")
        .author("Felipez <tifonzafel@gmail.com>")
        .about("Interact with a the APIs of auction sites")
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .help("URL to the endpoint")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("action")
                .help("Get or post from/to the URL")
                .default_value("get")
                .possible_values(&["get", "post"]),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name of the item"),
        )
        .subcommand(
            SubCommand::with_name("authenticate")
                .about("Authenticate the user")
                .arg(
                    Arg::with_name("token")
                        .short("t")
                        .help("Token to authenticate the user"),
                ),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let example = "https://api.bva-auctions.com/api/rest/ext123/auction/48048/summary";
    // match perform_transaction(example) {
    match deserialize_json() {
	Ok(()) => println!("42"),
	Err(msg) => println!("message: {}", msg)
    }
}
