extern crate postgres;

use serenity::model::prelude::UserId;
use serenity::prelude::TypeMapKey;
use std::env;
use std::sync::{Arc, Mutex};
// use std::time::{Duration, SystemTime};
use postgres::{Connection, TlsMode};

pub struct Settings(pub Arc<Mutex<Connection>>);

impl TypeMapKey for Settings {
    type Value = Settings;
}

impl Settings {
    pub fn new() -> Settings {
        let url = env::var("POSTGRES_URL").expect("Expected POSTGRES_URL to be set.");
        let connection = Connection::connect(url, TlsMode::None).unwrap();
        Settings(Arc::new(Mutex::new(connection)))
    }

    pub fn ensure_tables(&self) {
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
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS guilds (
                    id                                        BIGINT PRIMARY KEY,
                    prefix                                    VARCHAR(10),
                    tags                                      HSTORE,
                    channels_announcement_id                  BIGINT,
                    channels_greeting_id                      BIGINT,
                    channels_farewell_id                      BIGINT,
                    channels_member_logs_id                   BIGINT,
                    channels_message_logs_id                  BIGINT,
                    channels_nsfw_message_logs_id             BIGINT,
                    channels_moderation_logs_id               BIGINT,
                    channels_roles_id                         BIGINT,
                    channels_spam_id                          BIGINT,
                    command_autodelete                        HSTORE,
                    disabled_channels                         BIGINT[],
                    disabled_command_channels                 HSTORE,
                    events_ban_add                            BOOLEAN DEFAULT false,
                    events_ban_remove                         BOOLEAN DEFAULT false,
                    events_member_add                         BOOLEAN DEFAULT false,
                    events_member_remove                      BOOLEAN DEFAULT false,
                    events_message_add                        BOOLEAN DEFAULT false,
                    events_message_remove                     BOOLEAN DEFAULT false,
                    filter_level                              BIT(3) DEFAULT B'000',
                    filter_raw                                VARCHAR(100)[],
                    messages_farewell                         VARCHAR(2000),
                    messages_greeting                         VARCHAR(2000),
                    messages_join_dm                          VARCHAR(2000),
                    messages_warnings                         BOOLEAN DEFAULT false,
                    messages_ignore_channels                  BIGINT[],
                    sticky_roles                              HSTORE,
                    roles_administrator_id                    BIGINT,
                    roles_moderator_id                        BIGINT,
                    roles_staff_id                            BIGINT,
                    roles_automatic                           HSTORE,
                    roles_initial                             BIGINT,
                    roles_mute_id                             BIGINT,
                    roles_public                              BIGINT[],
                    roles_reactions                           HSTORE,
                    roles_remove_initial                      BOOLEAN DEFAULT false,
                    roles_subscriber_id                       BIGINT,
                    roles_unique_role_sets                    HSTORE,
                    selfmod_attachment                        BOOLEAN DEFAULT false,
                    selfmod_attachment_maximum                SMALLINT DEFAULT 20,
                    selfmod_attachment_duration               INTEGER DEFAULT 5000,
                    selfmod_attachment_action                 SMALLINT DEFAULT 0,
                    selfmod_attachment_punishment_duration    INTEGER,
                    selfmod_caps_enabled                      BOOLEAN,
                    selfmod_caps_minimum                      SMALLINT DEFAULT 10,
                    selfmod_caps_threshold                    SMALLINT DEFAULT 50,
                    selfmod_invitelinks_enabled               BOOLEAN,
                    selfmod_raid_enabled                      BOOLEAN,
                    selfmod_raid_threshold                    SMALLINT DEFAULT 10,
                    selfmod_ignore_channels                   BIGINT[],
                    nms_enabled                               BOOLEAN DEFAULT false,
                    nms_alert_enabled                         BOOLEAN DEFAULT false,
                    nms_allowed_mention_count                 SMALLINT DEFAULT 20,
                    nms_refresh_time                          SMALLINT DEFAULT 8,
                    social_achievement_enabled                BOOLEAN DEFAULT false,
                    social_achievement_message                VARCHAR(2000),
                    social_ignore_channels                    BIGINT[],
                    starboard_channel                         BIGINT,
                    starboard_emoji                           VARCHAR(4),
                    starboard_minimum_count                   SMALLINT DEFAULT 1,
                    starboard_ignore_channels                 BIGINT[],
                    trigger_alias                             HSTORE,
                    trigger_includes                          HSTORE
                )",
                &[],
            )
            .unwrap();
    }

    pub fn retrieve_user(&self, id: UserId) -> Option<UserSettings> {
        let connection = self.0.lock().unwrap();
        if let Ok(result) = connection.query("SELECT * FROM users WHERE id = $1", &[&(id.0 as i64)])
        {
            if result.is_empty() {
                None
            } else {
                let row = result.get(0);
                let next_daily: i64 = row.get(9);
                let next_reputation: i64 = row.get(10);
                Some(UserSettings {
                    id,
                    banner_set: row.get(1),
                    banner_list: row.get(2),
                    badge_set: row.get(3),
                    badge_list: row.get(4),
                    color: row.get(5),
                    money_count: row.get(6),
                    point_count: row.get(7),
                    reputation_count: row.get(8),
                    next_daily: next_daily as u64,
                    next_reputation: next_reputation as u64,
                })
            }
        } else {
            None
        }
    }

    pub fn try_daily(&self, id: UserId) -> Result<(), &str> {
        let connection = self.0.lock().unwrap();
        if let Ok(result) = connection.query(
            "SELECT next_daily FROM users WHERE id = $1",
            &[&(id.0 as i64)],
        ) {
            if result.is_empty() {
                connection
                    .execute(
                        "INSERT INTO users (id, money_count, next_daily)
                        VALUES ($1, $2, current_timestamp + interval '1 day')",
                        &[&(id.0 as i64), &200i32],
                    )
                    .expect("Failed to update database.");
                Ok(())
            } else {
                connection
                    .execute(
                        "UPDATE users
                        SET next_daily = current_timestamp + interval '1 day',
                            money_count = money_count + $2
                        WHERE id = $1",
                        &[&(id.0 as i64), &200i32],
                    )
                    .expect("Failed to update database.");
                Ok(())
            }
        } else {
            Err("The data retrieval from the database failed.")
        }
    }
}

// fn get_current_time() -> u64 {
//     SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards.").as_secs()
// }

#[derive(Debug)]
pub struct UserSettings {
    id: UserId,
    banner_set: String,
    banner_list: Vec<String>,
    badge_set: Vec<String>,
    badge_list: Vec<String>,
    color: u32,
    money_count: u32,
    point_count: u32,
    reputation_count: u32,
    next_daily: u64,
    next_reputation: u64,
}
