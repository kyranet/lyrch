extern crate bit_vec;
extern crate chrono;
extern crate dotenv;
extern crate postgres;
extern crate serde_json;
extern crate serenity;

mod commands;
mod lib;

use serenity::framework::standard::{DispatchError, StandardFramework};
use std::{collections::HashMap, env, sync::Arc};

use serenity::prelude::*;

fn main() {
    // Run dotenv first.
    dotenv::dotenv().unwrap();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let settings = lib::settings::Settings::new();
    settings.init();
    let mut client =
        Client::new(&token, lib::util::event_handlers::Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<lib::core::CommandCounter>(HashMap::default());
        data.insert::<lib::core::ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<lib::settings::Settings>(settings);
    }

    // We will fetch your bot's owners and id
    let (owners, bot_id) = lib::core::fetch_application_data(&client);

    client.with_framework(
        StandardFramework::new()
            .configure(|c| lib::core::configure(owners, bot_id, c))
            .before(move |ctx, msg, command_name| {
                println!(
                    "Got command '{}' by user '{}'",
                    command_name, msg.author.name
                );

                // Increment the number of times this command has been run once. If
                // the command's name does not exist in the counter, add a default
                // value of 0.
                let mut data = ctx.data.write();
                let stg = data.get::<lib::settings::Settings>().unwrap();
                if let Some(user) = stg.users.fetch(msg.author.id) {
                    println!("User Data: {:?}", user);
                }
                if let Some(guild_id) = msg.guild_id {
                    if let Some(guild) = stg.guilds.fetch(guild_id) {
                        println!("Guild Data: {:?}", guild);
                    }
                }
                if let Some(client) = stg.clients.fetch(bot_id) {
                    println!("Client Data: {:?}", client);
                }

                let counter = data
                    .get_mut::<lib::core::CommandCounter>()
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
            .bucket("social.profile", |b| b.delay(5).time_span(30).limit(2))
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
