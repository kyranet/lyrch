pub mod clients;
pub mod guilds;
pub mod users;

use postgres::{Connection, TlsMode};
use serenity::prelude::*;
use std::env;
use std::sync::{Arc, Mutex};

pub struct Settings {
    pub connection: Arc<Mutex<Connection>>,
    pub guilds: guilds::GuildSettingsHandler,
    pub users: users::UserSettingsHandler,
    pub clients: clients::ClientSettingsHandler,
}

impl TypeMapKey for Settings {
    type Value = Settings;
}

impl Settings {
    pub fn new() -> Settings {
        let url = env::var("POSTGRES_URL").expect("Expected POSTGRES_URL to be set.");
        let connection = Connection::connect(url, TlsMode::None).unwrap();
        let connection = Arc::new(Mutex::new(connection));
        Settings {
            connection: connection.clone(),
            guilds: guilds::GuildSettingsHandler::new(connection.clone()),
            users: users::UserSettingsHandler::new(connection.clone()),
            clients: clients::ClientSettingsHandler::new(connection.clone()),
        }
    }

    pub fn init(&self) {
        self.guilds.init();
        self.users.init();
        self.clients.init();
    }
}
