use super::SettingsHandler;
use postgres::Connection;
use serenity::model::prelude::*;
use std::sync::{Arc, Mutex};

pub struct ClientSettingsHandler(Arc<Mutex<Connection>>);

impl ClientSettingsHandler {
    pub fn new(connection: Arc<Mutex<Connection>>) -> ClientSettingsHandler {
        ClientSettingsHandler(connection)
    }
}

impl SettingsHandler for ClientSettingsHandler {
    type Id = UserId;
    type Output = ClientSettings;

    crate::apply_settings_init!(
        "clients",
        "
            id            BIGINT PRIMARY KEY,
            boosts_guild  BIGINT[]  DEFAULT '{}'::BIGINT[]  NOT NULL,
            boosts_users  BIGINT[]  DEFAULT '{}'::BIGINT[]  NOT NULL
        "
    );

    fn fetch(&self, id: impl AsRef<Self::Id>) -> Option<Self::Output> {
        let connection = self.0.lock().unwrap();
        let id = id.as_ref();
        if let Ok(result) = connection.query("SELECT * FROM users WHERE id = $1", &[&(id.0 as i64)])
        {
            if result.is_empty() {
                None
            } else {
                let row = result.get(0);
                Some(Self::Output {
                    id: *id,
                    boosts_guild: row.get(1),
                    boosts_users: row.get(2),
                })
            }
        } else {
            None
        }
    }

    crate::apply_settings_update!("clients");
    crate::apply_settings_update_increase!("clients");
}

#[derive(Debug)]
pub struct ClientSettings {
    pub id: UserId,
    pub boosts_guild: Vec<i64>,
    pub boosts_users: Vec<i64>,
}
