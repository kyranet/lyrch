use crate::try_send_message;
use serenity::{
    framework::standard::{
        help_commands,
        macros::{command, group, help},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
};
use std::collections::HashSet;

use serenity::prelude::*;

group!({
    name: "general",
    options: {},
    commands: [about, ping]
});

#[command]
#[only_in(guilds)]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let response = try_send_message!(ctx, msg, "Ping...");
    let latency = (response.timestamp - msg.timestamp).num_milliseconds();
    try_send_message!(ctx, msg, "Pong! Took: {}ms!", latency);
    Ok(())
}

#[command]
pub fn about(ctx: &mut Context, msg: &Message) -> CommandResult {
    try_send_message!(ctx, msg, "This is a small test-bot! :)");
    Ok(())
}

#[help]
#[individual_command_tip = "Hello! こんにちは！Hola! Bonjour! 您好!\n\
If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
pub fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::plain(context, msg, args, help_options, groups, owners)
}
