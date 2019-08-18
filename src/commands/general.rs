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

// The framework provides two built-in help commands for you to use.
// But you can also make your own customized help command that forwards
// to the behaviour of either of them.
#[help]
// This replaces the information that a user can pass
// a command-name as argument to gain specific information about it.
#[individual_command_tip = "Hello! こんにちは！Hola! Bonjour! 您好!\n\
If you want more information about a specific command, just pass the command as argument."]
// Some arguments require a `{}` in order to replace it with contextual information.
// In this case our `{}` refers to a command's name.
#[command_not_found_text = "Could not find: `{}`."]
// Define the maximum Levenshtein-distance between a searched command-name
// and commands. If the distance is lower than or equal the set distance,
// it will be displayed as a suggestion.
// Setting the distance to 0 will disable suggestions.
#[max_levenshtein_distance(3)]
// When you use sub-groups, Serenity will use the `indention_prefix` to indicate
// how deeply an item is indented.
// The default value is "-", it will be changed to "+".
#[indention_prefix = "+"]
// On another note, you can set up the help-menu-filter-behaviour.
// Here are all possible settings shown on all possible options.
// First case is if a user lacks permissions for a command, we can hide the command.
#[lacking_permissions = "Hide"]
// If the user is nothing but lacking a certain role, we just display it hence our variant is `Nothing`.
#[lacking_role = "Nothing"]
// The last `enum`-variant is `Strike`, which ~~strikes~~ a command.
#[wrong_channel = "Strike"]
// Serenity will automatically analyse and generate a hint/tip explaining the possible
// cases of ~~strikethrough-commands~~, but only if
// `strikethrough_commands_tip(Some(""))` keeps `Some()` wrapping an empty `String`, which is the default value.
// If the `String` is not empty, your given `String` will be used instead.
// If you pass in a `None`, no hint will be displayed at all.
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
