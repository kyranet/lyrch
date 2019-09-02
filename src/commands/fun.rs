use crate::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandError, CommandResult,
};

group!({
    name: "fun",
    options: {},
    commands: [choose]
});

#[command]
#[delimiters(",", ", ")]
pub fn choose(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let options = args.raw().collect::<Vec<&str>>();
    if options.len() <= 1 {
        return Err(CommandError(
            "Provide 2 or more choices, separated by commas.".into(),
        ));
    }
    // It is fine to run .unwrap here, since choose only Errs when the vector is empty
    let choice = options.choose(&mut thread_rng()).unwrap();
    try_send_message_content!(ctx, msg, "I think you should go with {}", choice)?;
    Ok(())
}
