use super::SettingsHandler;
use chrono::prelude::*;
use postgres::Connection;
use serenity::model::prelude::*;
use std::error::Error;
use std::sync::{Arc, Mutex};

pub struct UserSettingsHandler(Arc<Mutex<Connection>>);

impl UserSettingsHandler {
    pub fn new(connection: Arc<Mutex<Connection>>) -> UserSettingsHandler {
        UserSettingsHandler(connection)
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

    pub fn try_daily(&self, id: UserId) -> Result<(), String> {
        let id = &(id.0 as i64);
        let connection = self.0.lock().unwrap();
        let result = connection
            .query("SELECT next_daily FROM users WHERE id = $1", &[id])
            .map_err(|e| e.description().to_owned())?;

        if result.is_empty() {
            connection
                .execute(
                    "
                    INSERT INTO users (id, money_count, next_daily)
                    VALUES ($1, $2, current_timestamp + interval '1 day')",
                    &[id, &200i32],
                )
                .map_err(|e| e.description().to_owned())?;
            Ok(())
        } else {
            let row = result.get(0);
            let next_daily: Option<NaiveDateTime> = row.get(0);
            if let Some(time) = next_daily {
                let remaining = time - Utc::now().naive_utc();
                let seconds = remaining.num_seconds();
                if seconds > 0 {
                    let hours = remaining.num_hours();
                    let minutes = remaining.num_minutes();
                    return Err(format!(
                        "On cooldown. Remaining time: {}:{}:{}",
                        hours % 24,
                        minutes % 60,
                        seconds % 60
                    )
                    .to_owned());
                }
            }
            connection
                .execute(
                    "
                    UPDATE users
                    SET next_daily = current_timestamp + interval '1 day',
                        money_count = money_count + $2
                    WHERE id = $1",
                    &[id, &200i32],
                )
                .map_err(|e| e.description().to_owned())?;
            Ok(())
        }
    }
}

impl SettingsHandler for UserSettingsHandler {
    type Id = UserId;
    type Output = UserSettings;

    crate::apply_settings_init!(
        "users",
        "
            id                BIGINT PRIMARY KEY,
            banner_set        VARCHAR(6),
            banner_list       VARCHAR(6)[]  DEFAULT '{}'::VARCHAR(6)[]  NOT NULL,
            badge_set         VARCHAR(6)[]  DEFAULT '{}'::VARCHAR(6)[]  NOT NULL,
            badge_list        VARCHAR(6)[]  DEFAULT '{}'::VARCHAR(6)[]  NOT NULL,
            color             INTEGER       DEFAULT 0                   NOT NULL,
            money_count       INTEGER       DEFAULT 0                   NOT NULL,
            point_count       INTEGER       DEFAULT 0                   NOT NULL,
            reputation_count  INTEGER       DEFAULT 0                   NOT NULL,
            next_daily        TIMESTAMP,
            next_reputation   TIMESTAMP
        ",
        "points" => "
            point_count       DESC
        "
    );

    fn fetch(&self, id: impl AsRef<Self::Id>) -> Self::Output {
        let connection = self.0.lock().unwrap();
        let id = id.as_ref();
        if let Ok(result) = connection.query("SELECT * FROM users WHERE id = $1", &[&(id.0 as i64)])
        {
            if !result.is_empty() {
                let row = result.get(0);
                return Self::Output {
                    id: *id,
                    banner_set: row.get(1),
                    banner_list: row.get(2),
                    badge_set: row.get(3),
                    badge_list: row.get(4),
                    color: row.get(5),
                    money_count: row.get(6),
                    point_count: row.get(7),
                    reputation_count: row.get(8),
                    next_daily: row.get(9),
                    next_reputation: row.get(10),
                };
            }
        }

        Self::Output {
            id: *id,
            ..Self::Output::default()
        }
    }

    crate::apply_settings_update!("users");
    crate::apply_settings_update_increase!("users");
}

#[derive(Clone, Debug, Default)]
pub struct UserSettings {
    pub id: UserId,
    pub banner_set: Option<String>,
    pub banner_list: Vec<String>,
    pub badge_set: Vec<String>,
    pub badge_list: Vec<String>,
    pub color: i32,
    pub money_count: i32,
    pub point_count: i32,
    pub reputation_count: i32,
    pub next_daily: Option<NaiveDateTime>,
    pub next_reputation: Option<NaiveDateTime>,
}

impl UserSettings {
    pub fn get_level(&self) -> u32 {
        (0.2 * (self.point_count as f32).sqrt()).floor() as u32
    }
}
