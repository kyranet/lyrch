use postgres::Connection;
use serenity::model::prelude::*;
use std::sync::{Arc, Mutex};

pub struct ClientSettingsHandler(Arc<Mutex<Connection>>);

impl ClientSettingsHandler {
    pub fn new(connection: Arc<Mutex<Connection>>) -> ClientSettingsHandler {
        ClientSettingsHandler(connection)
    }

    pub fn init(&self) {
        let connection = self.0.lock().unwrap();
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS clientStorage (
                    id                  BIGINT PRIMARY KEY,
                    boosts_guild        BIGINT,
										boosts_users				BIGINT
                )",
                &[],
            )
            .unwrap();
    }

    pub fn fetch(&self, id: UserId) -> Option<ClientSettings> {
        let connection = self.0.lock().unwrap();
        if let Ok(result) = connection.query("SELECT * FROM clientStorage WHERE id = $1", &[&(id.0 as i64)])
        {
            if result.is_empty() {
                None
            } else {
								let row = result.get(0);
                Some(ClientSettings {
                    id,
										boosts_guild: row.get(1),
										boosts_users: row.get(2)
                })
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ClientSettings {
    pub id: UserId,
    pub boosts_guild: Option<i64>,
    pub boosts_users: Option<i64>,
}
