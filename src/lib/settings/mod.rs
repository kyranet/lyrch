pub mod clients;
pub mod guilds;
pub mod users;

use postgres::types::ToSql;
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use serenity::prelude::*;
use std::env;

pub struct Settings(pub Pool<PostgresConnectionManager>);

impl TypeMapKey for Settings {
    type Value = Settings;
}

impl Settings {
    pub fn new() -> Self {
        let url = env::var("POSTGRES_URL").expect("Expected POSTGRES_URL to be set.");
        let manager = PostgresConnectionManager::new(url, TlsMode::None).unwrap();
        let pool = Pool::new(manager).unwrap();
        Self(pool)
    }
}

pub trait SettingsHandler {
    type Id;
    type Output;

    fn init(&self) -> ();
    fn fetch(&self, id: impl AsRef<Self::Id>) -> Self::Output;
    fn update(
        &self,
        id: impl AsRef<Self::Id>,
        key: &str,
        value: &dyn ToSql,
    ) -> Result<(), postgres::Error>;
    fn update_increase(
        &self,
        id: impl AsRef<Self::Id>,
        key: &str,
        value: &dyn ToSql,
    ) -> Result<(), postgres::Error>;
}

#[macro_export]
macro_rules! apply_settings_init {
    ($table:expr, $schema:expr) => {
        fn init(&self) {
            let connection = self.0.clone().get().unwrap();
            connection
                .execute(
                    concat!("CREATE TABLE IF NOT EXISTS ", $table, " (\n", $schema, "\n)"),
                    &[]
                )
                .unwrap();
        }
    };
    ($table:expr, $schema:expr, $($index_name:tt => $index_content:tt)*) => {
        fn init(&self) {
            let connection = self.0.clone().get().unwrap();
            connection
                .execute(
                    concat!("CREATE TABLE IF NOT EXISTS ", $table, " (\n", $schema, "\n)"),
                    &[]
                )
                .unwrap();
            $(
                connection
                    .execute(
                        concat!(
                            "CREATE INDEX IF NOT EXISTS ",
                            $index_name,
                            " ON ONLY ",
                            $table,
                            " (\n",
                            $index_content,
                            "\n)"
                        ),
                        &[]
                    )
                    .unwrap();
            )*
        }
    };
}

#[macro_export]
macro_rules! apply_settings_update {
    ($table:expr) => {
        fn update(&self, id: impl AsRef<Self::Id>, key: &str, value: &dyn postgres::types::ToSql) -> Result<(), postgres::Error> {
            let connection = self.0.clone().get().unwrap();
            connection.execute(
                format!(concat!("INSERT INTO ", $table, " (id, {key})
                        VALUES ($1, $2)
                    ON CONFLICT (id)
                    DO UPDATE SET {key} = $2;"), key = key).as_str(),
                    &[&(id.as_ref().0 as i64), value]
            )?;
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! apply_settings_update_increase {
    ($table:expr) => {
        fn update_increase(&self, id: impl AsRef<Self::Id>, key: &str, value: &dyn postgres::types::ToSql) -> Result<(), postgres::Error> {
            let connection = self.0.clone().get().unwrap();
            connection.execute(
                format!(concat!("INSERT INTO ", $table, " (id, {key})
                        VALUES ($1, $2)
                    ON CONFLICT (id)
                    DO UPDATE SET {key} = users.{key} + $2;"), key = key).as_str(),
                    &[&(id.as_ref().0 as i64), value]
            )?;
            Ok(())
        }
    };
}
