use super::SettingsHandler;
use bit_vec::BitVec;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde_json::from_value;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;

pub struct GuildSettingsHandler(
    Pool<PostgresConnectionManager>,
    HashMap<GuildId, GuildSettings>,
);

impl TypeMapKey for GuildSettingsHandler {
    type Value = GuildSettingsHandler;
}

impl GuildSettingsHandler {
    pub fn new(connection: Pool<PostgresConnectionManager>) -> Self {
        Self(connection, HashMap::new())
    }

    pub fn get(&self, id: GuildId) -> Option<&GuildSettings> {
        self.1.get(&id)
    }

    pub fn add(&mut self, settings: GuildSettings) -> Option<GuildSettings> {
        self.1.insert(settings.id, settings)
    }

    pub fn remove(&mut self, id: GuildId) -> Option<GuildSettings> {
        self.1.remove(&id)
    }
}

impl SettingsHandler for GuildSettingsHandler {
    type Id = GuildId;
    type Output = GuildSettings;

    crate::apply_settings_init!(
        "guilds",
        "
            id                                      BIGINT          PRIMARY KEY,
            prefix                                  VARCHAR(10),
            language                                VARCHAR(5)      DEFAULT 'en_us'          NOT NULL,
            tags                                    JSON            DEFAULT '{}'::JSON       NOT NULL,
            channels_announcement_id                BIGINT,
            channels_greeting_id                    BIGINT,
            channels_farewell_id                    BIGINT,
            channels_member_logs_id                 BIGINT,
            channels_message_logs_id                BIGINT,
            channels_nsfw_message_logs_id           BIGINT,
            channels_moderation_logs_id             BIGINT,
            channels_roles_id                       BIGINT,
            channels_spam_id                        BIGINT,
            command_autodelete                      JSON            DEFAULT '{}'::JSON       NOT NULL,
            disabled_channels                       BIGINT[]        DEFAULT '{}'::BIGINT[]   NOT NULL,
            disabled_command_channels               JSON            DEFAULT '{}'::JSON       NOT NULL,
            events_ban_add                          BOOLEAN         DEFAULT false            NOT NULL,
            events_ban_remove                       BOOLEAN         DEFAULT false            NOT NULL,
            events_member_add                       BOOLEAN         DEFAULT false            NOT NULL,
            events_member_remove                    BOOLEAN         DEFAULT false            NOT NULL,
            events_message_add                      BOOLEAN         DEFAULT false            NOT NULL,
            events_message_remove                   BOOLEAN         DEFAULT false            NOT NULL,
            filter_level_enabled                    BIT(3)          DEFAULT B'000'           NOT NULL,
            filter_raw                              VARCHAR(100)[]  DEFAULT '{}'::BIGINT[]   NOT NULL,
            messages_farewell                       VARCHAR(2000),
            messages_greeting                       VARCHAR(2000),
            messages_join_dm                        VARCHAR(2000),
            messages_warnings                       BOOLEAN         DEFAULT false            NOT NULL,
            messages_ignore_channels                BIGINT[]        DEFAULT '{}'::BIGINT[]   NOT NULL,
            sticky_roles                            JSON            DEFAULT '{}'::JSON       NOT NULL,
            roles_administrator_id                  BIGINT,
            roles_moderator_id                      BIGINT,
            roles_staff_id                          BIGINT,
            roles_automatic                         JSON            DEFAULT '{}'::JSON       NOT NULL,
            roles_initial                           BIGINT,
            roles_mute_id                           BIGINT,
            roles_public                            BIGINT[]        DEFAULT '{}'::BIGINT[]   NOT NULL,
            roles_reactions                         JSON            DEFAULT '{}'::JSON       NOT NULL,
            roles_remove_initial                    BOOLEAN         DEFAULT false            NOT NULL,
            roles_subscriber_id                     BIGINT,
            roles_unique_role_sets                  JSON            DEFAULT '{}'::JSON       NOT NULL,
            selfmod_attachment                      BOOLEAN         DEFAULT false            NOT NULL,
            selfmod_attachment_maximum              SMALLINT        DEFAULT 20               NOT NULL,
            selfmod_attachment_duration             INTEGER         DEFAULT 5000             NOT NULL,
            selfmod_attachment_action               SMALLINT        DEFAULT 0                NOT NULL,
            selfmod_attachment_punishment_duration  INTEGER,
            selfmod_caps_enabled                    BIT(3)          DEFAULT B'000'           NOT NULL,
            selfmod_caps_minimum                    SMALLINT        DEFAULT 10               NOT NULL,
            selfmod_caps_threshold                  SMALLINT        DEFAULT 50               NOT NULL,
            selfmod_invitelinks_enabled             BIT(3)          DEFAULT B'000'           NOT NULL,
            selfmod_raid_enabled                    BIT(3)          DEFAULT B'000'           NOT NULL,
            selfmod_raid_threshold                  SMALLINT        DEFAULT 10               NOT NULL,
            selfmod_ignore_channels                 BIGINT[]        DEFAULT '{}'::BIGINT[]   NOT NULL,
            nms_enabled                             BOOLEAN         DEFAULT false            NOT NULL,
            nms_alert_enabled                       BOOLEAN         DEFAULT false            NOT NULL,
            nms_allowed_mention_count               SMALLINT        DEFAULT 20               NOT NULL,
            nms_refresh_time                        SMALLINT        DEFAULT 8                NOT NULL,
            social_achievement_enabled              BOOLEAN         DEFAULT false            NOT NULL,
            social_achievement_message              VARCHAR(2000),
            social_ignore_channels                  BIGINT[]        DEFAULT '{}'::BIGINT[]   NOT NULL,
            starboard_channel                       BIGINT,
            starboard_emoji                         VARCHAR(4),
            starboard_minimum_count                 SMALLINT        DEFAULT 1                NOT NULL,
            starboard_ignore_channels               BIGINT[]        DEFAULT '{}'::BIGINT[]   NOT NULL,
            trigger_alias                           JSON            DEFAULT '{}'::JSON       NOT NULL,
            trigger_includes                        JSON            DEFAULT '{}'::JSON       NOT NULL
        "
    );

    fn fetch(&self, id: impl AsRef<Self::Id>) -> Self::Output {
        let connection = self.0.clone().get().unwrap();
        let id = id.as_ref();
        if let Ok(result) =
            connection.query("SELECT * FROM guilds WHERE id = $1", &[&(id.0 as i64)])
        {
            if !result.is_empty() {
                let row = result.get(0);
                return Self::Output {
                    id: *id,
                    prefix: row.get(1),
                    language: row.get(2),
                    tags: from_value(row.get(3)).unwrap(),
                    channels_announcement_id: row.get(4),
                    channels_greeting_id: row.get(5),
                    channels_farewell_id: row.get(6),
                    channels_member_logs_id: row.get(7),
                    channels_message_logs_id: row.get(8),
                    channels_nsfw_message_logs_id: row.get(9),
                    channels_moderation_logs_id: row.get(10),
                    channels_roles_id: row.get(11),
                    channels_spam_id: row.get(12),
                    command_autodelete: from_value(row.get(13)).unwrap(),
                    disabled_channels: row.get(14),
                    disabled_command_channels: from_value(row.get(15)).unwrap(),
                    events_ban_add: row.get(16),
                    events_ban_remove: row.get(17),
                    events_member_add: row.get(18),
                    events_member_remove: row.get(19),
                    events_message_add: row.get(20),
                    events_message_remove: row.get(21),
                    filter_level_enabled: row.get(22),
                    filter_raw: row.get(23),
                    messages_farewell: row.get(24),
                    messages_greeting: row.get(25),
                    messages_join_dm: row.get(26),
                    messages_warnings: row.get(27),
                    messages_ignore_channels: row.get(28),
                    sticky_roles: from_value(row.get(29)).unwrap(),
                    roles_administrator_id: row.get(30),
                    roles_moderator_id: row.get(31),
                    roles_staff_id: row.get(32),
                    roles_automatic: from_value(row.get(33)).unwrap(),
                    roles_initial: row.get(34),
                    roles_mute_id: row.get(35),
                    roles_public: row.get(36),
                    roles_reactions: from_value(row.get(37)).unwrap(),
                    roles_remove_initial: row.get(38),
                    roles_subscriber_id: row.get(39),
                    roles_unique_role_sets: from_value(row.get(40)).unwrap(),
                    selfmod_attachment: row.get(41),
                    selfmod_attachment_maximum: row.get(42),
                    selfmod_attachment_duration: row.get(43),
                    selfmod_attachment_action: row.get(44),
                    selfmod_attachment_punishment_duration: row.get(45),
                    selfmod_caps_enabled: row.get(46),
                    selfmod_caps_minimum: row.get(47),
                    selfmod_caps_threshold: row.get(48),
                    selfmod_invitelinks_enabled: row.get(49),
                    selfmod_raid_enabled: row.get(50),
                    selfmod_raid_threshold: row.get(51),
                    selfmod_ignore_channels: row.get(52),
                    nms_enabled: row.get(53),
                    nms_alert_enabled: row.get(54),
                    nms_allowed_mention_count: row.get(55),
                    nms_refresh_time: row.get(56),
                    social_achievement_enabled: row.get(57),
                    social_achievement_message: row.get(58),
                    social_ignore_channels: row.get(59),
                    starboard_channel: row.get(60),
                    starboard_emoji: row.get(61),
                    starboard_minimum_count: row.get(62),
                    starboard_ignore_channels: row.get(63),
                    trigger_alias: from_value(row.get(64)).unwrap(),
                    trigger_includes: from_value(row.get(65)).unwrap(),
                };
            };
        }

        Self::Output {
            id: *id,
            language: "en_us".to_owned(),
            ..Self::Output::default()
        }
    }

    crate::apply_settings_insert!("guilds");
    crate::apply_settings_update!("guilds");
    crate::apply_settings_update_increase!("guilds");
}

#[derive(Clone, Debug, Default)]
pub struct GuildSettings {
    pub id: GuildId,
    pub prefix: Option<String>,
    pub language: String,
    pub tags: HashMap<String, String>,
    pub channels_announcement_id: Option<i64>,
    pub channels_greeting_id: Option<i64>,
    pub channels_farewell_id: Option<i64>,
    pub channels_member_logs_id: Option<i64>,
    pub channels_message_logs_id: Option<i64>,
    pub channels_nsfw_message_logs_id: Option<i64>,
    pub channels_moderation_logs_id: Option<i64>,
    pub channels_roles_id: Option<i64>,
    pub channels_spam_id: Option<i64>,
    pub command_autodelete: HashMap<i64, u32>,
    pub disabled_channels: Vec<i64>,
    pub disabled_command_channels: HashMap<u64, Vec<String>>,
    pub events_ban_add: bool,
    pub events_ban_remove: bool,
    pub events_member_add: bool,
    pub events_member_remove: bool,
    pub events_message_add: bool,
    pub events_message_remove: bool,
    pub filter_level_enabled: BitVec,
    pub filter_raw: Vec<String>,
    pub messages_farewell: Option<String>,
    pub messages_greeting: Option<String>,
    pub messages_join_dm: Option<String>,
    pub messages_warnings: bool,
    pub messages_ignore_channels: Vec<i64>,
    pub sticky_roles: HashMap<UserId, Vec<i64>>,
    pub roles_administrator_id: Option<i64>,
    pub roles_moderator_id: Option<i64>,
    pub roles_staff_id: Option<i64>,
    pub roles_automatic: HashMap<i64, u32>,
    pub roles_initial: Option<i64>,
    pub roles_mute_id: Option<i64>,
    pub roles_public: Vec<i64>,
    pub roles_reactions: HashMap<String, i64>,
    pub roles_remove_initial: bool,
    pub roles_subscriber_id: Option<i64>,
    pub roles_unique_role_sets: HashMap<String, Vec<String>>,
    pub selfmod_attachment: bool,
    pub selfmod_attachment_maximum: i16,
    pub selfmod_attachment_duration: i32,
    pub selfmod_attachment_action: i16,
    pub selfmod_attachment_punishment_duration: Option<i32>,
    pub selfmod_caps_enabled: BitVec,
    pub selfmod_caps_minimum: i16,
    pub selfmod_caps_threshold: i16,
    pub selfmod_invitelinks_enabled: BitVec,
    pub selfmod_raid_enabled: BitVec,
    pub selfmod_raid_threshold: i16,
    pub selfmod_ignore_channels: Vec<i64>,
    pub nms_enabled: bool,
    pub nms_alert_enabled: bool,
    pub nms_allowed_mention_count: i16,
    pub nms_refresh_time: i16,
    pub social_achievement_enabled: bool,
    pub social_achievement_message: Option<String>,
    pub social_ignore_channels: Vec<i64>,
    pub starboard_channel: Option<i64>,
    pub starboard_emoji: Option<String>,
    pub starboard_minimum_count: i16,
    pub starboard_ignore_channels: Vec<i64>,
    pub trigger_alias: HashMap<String, String>,
    pub trigger_includes: HashMap<String, String>,
}
