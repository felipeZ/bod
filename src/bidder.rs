extern crate reqwest;

use reqwest::Client;
use std::collections::VecDeque;
use std::time::SystemTime;

struct Message {
    text: String,
    timestamp: SystemTime,
}

pub struct Bidder {
    url: String,
    requests: VecDeque<Message>,
    replies: VecDeque<Message>,
    client: Option<Client>,
}

impl Bidder {
    pub fn new(url: String) -> Self {
        Bidder {
            url: url,
            requests: VecDeque::new(),
            replies: VecDeque::new(),
	    client: None,
        }
    }
}
