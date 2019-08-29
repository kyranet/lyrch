use crate::try_send_message_content;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult, CommandError
    },
    model::prelude::*,
    prelude::*,
};
use rand::thread_rng;
use rand::seq::SliceRandom;

group!({
    name: "fun",
    options: {},
    commands: [choose]
});

#[command]
pub fn choose(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let options: Vec<&str> = args.rest().split(", ").collect();
    if options.len() <= 1 {
        return Err(CommandError("Provide 2 or more choices, separated by commas.".into()));
    }
    // TODO: is it safe to call this unwrap?
    let choice = options.choose(&mut thread_rng()).unwrap();
    try_send_message_content!(ctx, msg, "I think you should go with {}", choice)?;
    Ok(())
}