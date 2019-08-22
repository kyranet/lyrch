use super::SettingsHandler;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub struct ClientSettingsHandler(pub Pool<PostgresConnectionManager>);

impl ClientSettingsHandler {
    pub fn new(pool: Pool<PostgresConnectionManager>) -> Self {
        Self(pool)
    }
}

impl TypeMapKey for ClientSettingsHandler {
    type Value = ClientSettingsHandler;
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

    crate::apply_settings_fetch!("clients");
    crate::apply_settings_update!("clients");
    crate::apply_settings_update_increase!("clients");
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ClientSettings {
    pub id: UserId,
    pub boosts_guild: Vec<i64>,
    pub boosts_users: Vec<i64>,
}
