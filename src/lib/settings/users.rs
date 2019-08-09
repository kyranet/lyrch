use serenity::model::prelude::*;
use chrono::prelude::*;
use postgres::Connection;
use std::sync::{Arc, Mutex};

pub struct UserSettingsHandler(Arc<Mutex<Connection>>);

impl UserSettingsHandler {
    pub fn new(connection: Arc<Mutex<Connection>>) -> UserSettingsHandler {
        UserSettingsHandler(connection)
    }

    pub fn init(&self) {
        let connection = self.0.lock().unwrap();
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS users (
                    id                  BIGINT PRIMARY KEY,
                    banner_set          VARCHAR(6),
                    banner_list         VARCHAR(6)[],
                    badge_set           VARCHAR(6)[],
                    badge_list          VARCHAR(6)[],
                    color               INTEGER DEFAULT 0,
                    money_count         INTEGER DEFAULT 0,
                    point_count         INTEGER DEFAULT 0,
                    reputation_count    INTEGER DEFAULT 0,
                    next_daily          TIMESTAMP,
                    next_reputation     TIMESTAMP
                )",
                &[],
            )
            .unwrap();
        connection
            .execute(
                "CREATE INDEX IF NOT EXISTS points ON ONLY users (
                    point_count         DESC
                )",
                &[],
            )
            .unwrap();
    }

    pub fn fetch(&self, id: UserId) -> Option<UserSettings> {
        let connection = self.0.lock().unwrap();
        if let Ok(result) = connection.query("SELECT * FROM users WHERE id = $1", &[&(id.0 as i64)])
        {
            if result.is_empty() {
                None
            } else {
                let row = result.get(0);
                let color: i32 = row.get(5);
                let money_count: i32 = row.get(6);
                let point_count: i32 = row.get(7);
                let reputation_count: i32 = row.get(8);
                Some(UserSettings {
                    id,
                    banner_set: row.get(1),
                    banner_list: row.get(2),
                    badge_set: row.get(3),
                    badge_list: row.get(4),
                    color: color as u32,
                    money_count: money_count as u32,
                    point_count: point_count as u32,
                    reputation_count: reputation_count as u32,
                    next_daily: row.get(9),
                    next_reputation: row.get(10),
                })
            }
        } else {
            None
        }
    }

    pub fn retrieve_user_money_count(&self, id: UserId) -> u32 {
        let connection = self.0.lock().unwrap();
        if let Ok(result) = connection.query(
            "SELECT money_count FROM users WHERE id = $1",
            &[&(id.0 as i64)],
        ) {
            if result.is_empty() {
                0
            } else {
                let row = result.get(0);
                let money_count: i32 = row.get(0);
                money_count as u32
            }
        } else {
            0
        }
    }

    pub fn try_daily(&self, id: UserId) -> Result<(), &str> {
        let connection = self.0.lock().unwrap();
        if let Ok(result) = connection.query(
            "SELECT next_daily FROM users WHERE id = $1",
            &[&(id.0 as i64)],
        ) {
            // Create if not exists
            if result.is_empty() {
                return if try_daily_create(&connection, &(id.0 as i64)) {
                    Ok(())
                } else {
                    Err("Failed to update database.")
                };
            }

            let row = result.get(0);
            return if try_daily_update(&connection, &(id.0 as i64), row.get(0)) {
                Ok(())
            } else {
                Err("You have claimed dailies too early.")
            };
        }
        Err("The data retrieval from the database failed.")
    }
}

fn try_daily_create(connection: &Connection, id: &i64) -> bool {
    connection
        .execute(
            "INSERT INTO users (id, money_count, next_daily)
        VALUES ($1, $2, current_timestamp + interval '1 day')",
            &[id, &200i32],
        )
        .is_ok()
}

fn try_daily_update(connection: &Connection, id: &i64, next_daily: Option<NaiveDateTime>) -> bool {
    if let Some(time) = next_daily {
        if time > Utc::now().naive_utc() {
            return false;
        }
    }
    connection
        .execute(
            "UPDATE users
        SET next_daily = current_timestamp + interval '1 day',
            money_count = money_count + $2
        WHERE id = $1",
            &[id, &200i32],
        )
        .is_ok()
}

#[derive(Debug)]
pub struct UserSettings {
    pub id: UserId,
    pub banner_set: Option<String>,
    pub banner_list: Option<Vec<String>>,
    pub badge_set: Option<Vec<String>>,
    pub badge_list: Option<Vec<String>>,
    pub color: u32,
    pub money_count: u32,
    pub point_count: u32,
    pub reputation_count: u32,
    pub next_daily: Option<NaiveDateTime>,
    pub next_reputation: Option<NaiveDateTime>,
}
