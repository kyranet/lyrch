use crate::commands;
use crate::lib;
use crate::lib::settings::SettingsHandler;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;
use threadpool::ThreadPool;

pub fn initialize_client() -> Client {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    Client::new(&token, lib::util::event_handlers::Handler).expect("Err creating client")
}

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

pub fn create_framework(owners: HashSet<UserId>, bot_id: UserId) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| configure(owners, bot_id, c))
        .before(move |ctx, msg, command_name| {
            crate::debug!(
                "Got command '{}' by user '{}'",
                command_name,
                msg.author.name
            );

            {
                let mut data = ctx.data.write();
                let counter = data
                    .get_mut::<lib::core::CommandCounter>()
                    .expect("Expected CommandCounter in ShareMap.");
                let entry = counter.entry(command_name.to_string()).or_insert(0);
                *entry += 1;
            }

            crate::monitors::run(ctx, msg)
        })
        // Similar to `before`, except will be called directly _after_
        // command execution.
        .after(|ctx, msg, command_name, error| {
            if let Err(why) = error {
                crate::wtf!("Command '{}' returned error {:?}", command_name, why);
                let _ = msg.channel_id.say(
                    &ctx.http,
                    &format!("Whoops! I'm very sorry, something happened: `{}`", why.0),
                );
            }
        })
        .prefix_only(move |ctx, msg| {
            if let Some(user) = msg.mentions.first() {
                if user.id == bot_id {
                    msg.channel_id
                        .say(
                            &ctx.http,
                            &format!("The prefix is `{}`", env::var("PREFIX").unwrap()),
                        )
                        .ok();
                }
            }
        })
        // Set a function that's called whenever an attempted command-call's
        // command could not be found.
        .unrecognised_command(|_, _, unknown_command_name| {
            crate::verbose!("Could not find command named '{}'", unknown_command_name);
        })
        // Set a function that's called whenever a message is not a command.
        .normal_message(|ctx, msg| {
            crate::monitors::run(ctx, msg);
        })
        // Set a function that's called whenever a command's execution didn't complete for one
        // reason or another. For example, when a user has exceeded a rate-limit or a command
        // can only be performed by the bot owner.
        .on_dispatch_error(|ctx, msg, error| {
            if let DispatchError::Ratelimited(seconds) = error {
                let _ = msg.channel_id.say(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", seconds),
                );
            }
        })
        .help(&commands::general::MY_HELP)
        // Can't be used more than once per 5 seconds:
        // .bucket("emoji", |b| b.delay(5))
        // Can't be used more than 2 times per 30 seconds, with a 5 second delay:
        .bucket("social.profile", |b| b.delay(5).time_span(30).limit(2))
        // The `group!` macro generates `static` instances of the options set for the group.
        // They're made in the pattern: `#name_GROUP` for the group instance and `#name_GROUP_OPTIONS`.
        // #name is turned all uppercase
        .group(&commands::animal::ANIMAL_GROUP)
        .group(&commands::general::GENERAL_GROUP)
        .group(&commands::misc::MISC_GROUP)
        .group(&commands::social::SOCIAL_GROUP)
        .group(&commands::weeb::WEEB_GROUP)
        .group(&commands::fun::FUN_GROUP)
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
                let guilds = data
                    .get::<lib::settings::guilds::GuildSettingsHandler>()
                    .unwrap();
                if let Some(guild) = guilds.get(guild_id) {
                    return guild.prefix.clone();
                }
            }

            None
        })
        .delimiters(vec![", ", ",", " "])
        .owners(owners)
}

pub fn attach_data(client: &mut Client, framework: lib::framework::LyrchFramework) {
    use lib::{
        cache, core, framework,
        schedule::{self, DispatcherKey, SchedulerKey},
        settings::{
            clients::ClientSettingsHandler, guilds::GuildSettingsHandler,
            reminders::RemindersSettingsHandler, users::UserSettingsHandler, Settings,
        },
    };

    if let Ok(amount) = env::var("THREADS")
        .unwrap_or_else(|_| "5".to_owned())
        .parse::<usize>()
    {
        client.threadpool.set_num_threads(amount);
    }

    let settings = Settings::new();
    let mut data = client.data.write();
    data.insert::<ClientSettingsHandler>(ClientSettingsHandler::new(settings.0.clone()).init());
    data.insert::<GuildSettingsHandler>(GuildSettingsHandler::new(settings.0.clone()).init());
    data.insert::<UserSettingsHandler>(UserSettingsHandler::new(settings.0.clone()).init());
    data.insert::<RemindersSettingsHandler>(
        RemindersSettingsHandler::new(settings.0.clone()).init(),
    );
    data.insert::<Settings>(settings);
    data.insert::<cache::RedisConnection>(lib::cache::RedisConnection::new());
    data.insert::<core::CommandCounter>(HashMap::default());
    data.insert::<core::ShardManagerContainer>(Arc::clone(&client.shard_manager));
    data.insert::<core::ThreadPoolContainer>(Arc::new(Mutex::new(client.threadpool.clone())));
    data.insert::<framework::LyrchFramework>(framework);
    data.insert::<DispatcherKey>(schedule::create_dispatcher());
    data.insert::<SchedulerKey>(schedule::create_scheduler());
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

pub struct ThreadPoolContainer;

impl TypeMapKey for ThreadPoolContainer {
    type Value = Arc<Mutex<ThreadPool>>;
}
