use super::SettingsHandler;
use crate::lib::internal::ScheduleId;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub struct RemindersSettingsHandler(pub Pool<PostgresConnectionManager>);

impl RemindersSettingsHandler {
    pub fn new(pool: Pool<PostgresConnectionManager>) -> Self {
        Self(pool)
    }
}

impl TypeMapKey for RemindersSettingsHandler {
    type Value = RemindersSettingsHandler;
}

impl SettingsHandler for RemindersSettingsHandler {
    type Id = ScheduleId;
    type Output = SchedulesSettings;

    crate::apply_settings_init!(
        "reminders",
        "
            id       BIGINT PRIMARY KEY,
            user_id  BIGINT              NOT NULL,
            content  VARCHAR(250)        NOT NULL
        "
    );

    crate::apply_settings_fetch!("schedules");
    crate::apply_settings_update!("schedules");
    crate::apply_settings_update_increase!("schedules");
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SchedulesSettings {
    pub id: ScheduleId,
    pub user_id: UserId,
    pub content: String,
}
