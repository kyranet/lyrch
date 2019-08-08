use crate::lib::settings::Settings;
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

use serenity::prelude::*;

group!({
    name: "social",
    options: {},
    commands: [daily]
});

#[command]
pub fn daily(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let settings = data.get::<Settings>().unwrap();
    if let Err(why) = match settings.try_daily(msg.author.id) {
        Ok(()) => msg
            .channel_id
            .say(&ctx.http, "Yay! You received 200 shinies!"),
        Err(err) => msg
            .channel_id
            .say(&ctx.http, format!("Whoops! Something happened! {}", err)),
    } {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
