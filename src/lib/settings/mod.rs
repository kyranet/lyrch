pub mod clients;
pub mod guilds;
pub mod reminders;
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

    fn init(self) -> Self;
    fn fetch(&self, id: impl AsRef<Self::Id>) -> Self::Output;
    fn insert(
        &self,
        id: impl AsRef<Self::Id>,
        keys: &[&'static str],
        values: &[&dyn postgres::types::ToSql],
    ) -> Result<(), postgres::Error>;
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
        fn init(self) -> Self {
            let connection = self.0.clone().get().unwrap();
            connection
                .execute(
                    concat!("CREATE TABLE IF NOT EXISTS ", $table, " (\n", $schema, "\n)"),
                    &[]
                )
                .unwrap();
            self
        }
    };
    ($table:expr, $schema:expr, $($index_name:tt => $index_content:tt)*) => {
        fn init(self) -> Self {
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
            self
        }
    };
}

#[macro_export]
macro_rules! apply_settings_fetch {
    ($table:expr) => {
        fn fetch(&self, id: impl AsRef<Self::Id>) -> Self::Output {
            let connection = self.0.clone().get().unwrap();
            let id = id.as_ref();
            if let Ok(result) = connection.query(concat!("SELECT * FROM ", $table, " WHERE id = $1"), &[&(id.0 as i64)])
            {
                if !result.is_empty() {
                    serde_postgres::from_row(result.get(0)).unwrap()
                }
            }

            Self::Output { id: *id, ..Self::Output::default() }
        }
    };
}

#[macro_export]
macro_rules! apply_settings_insert {
    ($table:expr) => {
        fn insert(
            &self,
            id: impl AsRef<Self::Id>,
            keys: &[&'static str],
            values: &[&dyn postgres::types::ToSql],
        ) -> Result<(), postgres::Error> {
            use std::fmt::Write;
            let connection = self.0.clone().get().unwrap();

            let mut i = 2;
            let mut map_keys = String::new();
            let mut map_params = String::new();
            let mut keys_iter = keys.iter();
            if let Some(key) = keys_iter.next() {
                write!(&mut map_keys, "{}", key).unwrap();
                write!(&mut map_params, "${}", i).unwrap();

                for key in keys_iter {
                    i += 1;
                    write!(&mut map_keys, ", {}", key).unwrap();
                    write!(&mut map_params, ", ${}", i).unwrap();
                }
            }

            let mut expanded_values: Vec<&dyn postgres::types::ToSql> = Vec::with_capacity(keys.len());
            let id = id.as_ref().0 as i64;
            expanded_values.push(&id);
            expanded_values.extend(values);
            connection.execute(
                format!(concat!("INSERT INTO ", $table, " (id, {keys})
                        VALUES ($1, {params});"), keys = map_keys, params = map_params).as_str(),
                    &expanded_values
            )?;
            Ok(())
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
