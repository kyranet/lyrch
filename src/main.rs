#[macro_use]
extern crate lazy_static;

mod commands;
mod i18n;
mod lib;
mod monitors;
mod prelude;
mod tasks;

use lib::core::{attach_data, create_framework, fetch_application_data, initialize_client};

fn main() {
    // Run dotenv first.
    dotenv::dotenv().unwrap();

    let mut client = initialize_client();
    let (owners, bot_id) = fetch_application_data(&client);
    let framework = lib::framework::LyrchFramework::new(create_framework(owners, bot_id));
    attach_data(&mut client, framework.clone());
    client.with_framework(framework);

    if let Err(why) = client.start() {
        crate::wtf!("Client error: {:?}", why);
    }
}
