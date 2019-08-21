extern crate bit_vec;
extern crate chrono;
extern crate dotenv;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate r2d2_redis;
extern crate redis;
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
    let framework = lib::framework::LyrchFramework::new(create_framework(owners, bot_id));
    attach_data(&client, framework.clone());
    client.with_framework(framework);

    if let Err(why) = client.start() {
        crate::wtf!("Client error: {:?}", why);
    }
}
