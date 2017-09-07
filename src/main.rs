extern crate pushover;

use std::env;
use std::process;
use pushover::SyncAPI;
use pushover::requests::message::SendMessage;

fn main() {
    let key = env!("PUSHOVER_KEY");
    let token = env!("PUSHOVER_TOKEN");
    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 1 {
        process::exit(0);
    }
    
    let api = SyncAPI::new().expect("Error creating API");
    
    
    let msg = SendMessage::new(token, key, args[1].to_string());

    let response = api.send(&msg);
    println!("{:?}", response);
}
