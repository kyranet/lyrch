extern crate bit_vec;
extern crate chrono;
extern crate dotenv;
extern crate postgres;
extern crate serde_json;
extern crate serenity;

mod commands;
mod lib;

use serenity::model::prelude::*;
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{DispatchError, StandardFramework},
    model::gateway::Ready,
};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};

// This imports `typemap`'s `Key` as `TypeMapKey`.
use lib::settings::Settings;
use serenity::prelude::*;

// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        let mut data = ctx.data.write();
        let settings = data.get_mut::<Settings>().unwrap();
        if let Some(guild_settings) = settings.guilds.fetch(guild.id) {
            settings.guilds.add(guild_settings);
        }
    }

    fn guild_delete(
        &self,
        ctx: Context,
        incomplete: PartialGuild,
        _full: Option<Arc<RwLock<Guild>>>,
    ) {
        let mut data = ctx.data.write();
        let settings = data.get_mut::<Settings>().unwrap();
        settings.guilds.remove(incomplete.id);
    }
}

fn main() {
    // Run dotenv first.
    dotenv::dotenv().unwrap();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let settings = Settings::new();
    settings.init();
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<Settings>(settings);
    }

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    client.with_framework(
        // Configures the client, allowing for options to mutate how the
        // framework functions.
        //
        // Refer to the documentation for
        // `serenity::ext::framework::Configuration` for all available
        // configurations.
        StandardFramework::new()
            .configure(|c| {
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
                    // You can set multiple delimiters via delimiters()
                    // or just one via delimiter(",")
                    // If you set multiple delimiters, the order you list them
                    // decides their priority (from first to last).
                    //
                    // In this case, if "," would be first, a message would never
                    // be delimited at ", ", forcing you to trim your arguments if you
                    // want to avoid whitespaces at the start of each.
                    .delimiters(vec![", ", ",", " "])
                    // Sets the bot's owners. These will be used for commands that
                    // are owners only.
                    .owners(owners)
            })
            // Set a function to be called prior to each command execution. This
            // provides the context of the command, the message that was received,
            // and the full name of the command that will be called.
            //
            // You can not use this to determine whether a command should be
            // executed. Instead, the `#[check]` macro gives you this functionality.
            .before(move |ctx, msg, command_name| {
                println!(
                    "Got command '{}' by user '{}'",
                    command_name, msg.author.name
                );

                // Increment the number of times this command has been run once. If
                // the command's name does not exist in the counter, add a default
                // value of 0.
                let mut data = ctx.data.write();
                let stg = data.get::<Settings>().unwrap();
                if let Some(user) = stg.users.fetch(msg.author.id) {
                    println!("User Data: {:?}", user);
                }
                if let Some(guild_id) = msg.guild_id {
                    if let Some(guild) = stg.guilds.fetch(guild_id) {
                        println!("Guild Data: {:?}", guild);
                    }
                }
                if let Some(client) = stg.client.fetch(bot_id) {
                    println!("Guild Data: {:?}", client);
                }

                let counter = data
                    .get_mut::<CommandCounter>()
                    .expect("Expected CommandCounter in ShareMap.");
                let entry = counter.entry(command_name.to_string()).or_insert(0);
                *entry += 1;

                true // if `before` returns false, command processing doesn't happen.
            })
            // Similar to `before`, except will be called directly _after_
            // command execution.
            .after(|_, _, command_name, error| match error {
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
            })
            .prefix_only(move |ctx, message| {
                if let Some(user) = message.mentions.first() {
                    if user.id == bot_id {
                        message
                            .channel_id
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
                println!("Could not find command named '{}'", unknown_command_name);
            })
            // Set a function that's called whenever a message is not a command.
            .normal_message(|_, message| {
                println!("Message is not a command '{}'", message.content);
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
            .bucket("complicated", |b| b.delay(5).time_span(30).limit(2))
            // The `group!` macro generates `static` instances of the options set for the group.
            // They're made in the pattern: `#name_GROUP` for the group instance and `#name_GROUP_OPTIONS`.
            // #name is turned all uppercase
            .group(&commands::general::GENERAL_GROUP)
            .group(&commands::social::SOCIAL_GROUP), // .group(&EMOJI_GROUP)
                                                     // .group(&MATH_GROUP)
                                                     // .group(&OWNER_GROUP)
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
