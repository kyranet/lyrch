use crate::lib::settings::Settings;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;

pub fn fetch_application_data(client: &Client) -> (HashSet<UserId>, UserId) {
    match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    }
}

pub fn configure(
    owners: HashSet<UserId>,
    bot_id: UserId,
    c: &mut Configuration,
) -> &mut Configuration {
    c.with_whitespace(true)
        .on_mention(Some(bot_id))
        .no_dm_prefix(false)
        .case_insensitivity(false)
        .prefix(
            env::var("PREFIX")
                .expect("A prefix must be configured.")
                .as_ref(),
        )
        .dynamic_prefix(|ctx, msg| {
            if let Some(guild_id) = msg.guild_id {
                let data = ctx.data.write();
                let stg = data.get::<Settings>().unwrap();
                if let Some(guild) = stg.guilds.get(guild_id) {
                    return guild.prefix.clone();
                }
            }

            None
        })
        .delimiters(vec![", ", ",", " "])
        .owners(owners)
}

// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}
