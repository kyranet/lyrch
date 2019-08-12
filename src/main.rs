extern crate bit_vec;
extern crate chrono;
extern crate dotenv;
extern crate postgres;
extern crate serde_json;
extern crate serenity;

mod commands;
mod lib;

use lib::core::{create_framework, fetch_application_data, initialize_client};

fn main() {
    // Run dotenv first.
    dotenv::dotenv().unwrap();

    let mut client = initialize_client();
    let (owners, bot_id) = fetch_application_data(&client);
    client.with_framework(create_framework(owners, bot_id));

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
