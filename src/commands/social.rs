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
    commands: [daily, credits]
});

#[command]
pub fn daily(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let settings = data.get::<Settings>().unwrap();
    if let Err(why) = match settings.users.try_daily(msg.author.id) {
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

#[command]
pub fn credits(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let settings = data.get::<Settings>().unwrap();
    let amount = settings.users.retrieve_user_money_count(msg.author.id);
    if let Err(why) = msg.channel_id.say(
        &ctx.http,
        format!(
            "You have a total of {}<:ShinyYellow:324157128270938113>",
            amount
        ),
    ) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
