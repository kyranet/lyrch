use crate::try_send_message_content;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandError, CommandResult,
    },
    model::prelude::*,
    prelude::*,
};

group!({
    name: "fun",
    options: {},
    commands: [choose]
});

#[command]
pub fn choose(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let options: Vec<&str> = args.rest().split(", ").collect();
    if options.len() <= 1 {
        return Err(CommandError(
            "Provide 2 or more choices, separated by commas.".into(),
        ));
    }
    // TODO: is it safe to call this unwrap?
    let choice = options.choose(&mut thread_rng()).unwrap();
    try_send_message_content!(ctx, msg, "I think you should go with {}", choice)?;
    Ok(())
}
