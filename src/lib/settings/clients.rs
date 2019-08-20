use super::SettingsHandler;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
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

    fn fetch(&self, id: impl AsRef<Self::Id>) -> Self::Output {
        let connection = self.0.clone().get().unwrap();
        let id = id.as_ref();
        if let Ok(result) = connection.query("SELECT * FROM users WHERE id = $1", &[&(id.0 as i64)])
        {
            if !result.is_empty() {
                let row = result.get(0);
                return Self::Output {
                    id: *id,
                    boosts_guild: row.get(1),
                    boosts_users: row.get(2),
                };
            }
        }

        Self::Output {
            id: *id,
            ..Self::Output::default()
        }
    }

    crate::apply_settings_update!("clients");
    crate::apply_settings_update_increase!("clients");
}

#[derive(Clone, Debug, Default)]
pub struct ClientSettings {
    pub id: UserId,
    pub boosts_guild: Vec<i64>,
    pub boosts_users: Vec<i64>,
}
