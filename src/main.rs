extern crate bit_vec;
extern crate chrono;
extern crate dotenv;
extern crate postgres;
extern crate regex;
extern crate serde_json;
extern crate serenity;
#[macro_use]
extern crate lazy_static;

mod commands;
mod lib;
mod monitors;

use lib::core::{attach_data, create_framework, fetch_application_data, initialize_client};

fn main() {
    // Run dotenv first.
    dotenv::dotenv().unwrap();

    let mut client = initialize_client();
    let (owners, bot_id) = fetch_application_data(&client);
    let framework = create_framework(owners, bot_id);
    attach_data(&client);
    client.with_framework(framework);

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
