use crate::lib::settings::Settings;
use crate::lib::util::percentage;
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
    commands: [daily, credits, profile]
});

#[command]
pub fn daily(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let settings = data.get::<Settings>().unwrap();
    if let Err(why) = match settings.users.try_daily(msg.author.id) {
        Ok(()) => msg
            .channel_id
            .say(&ctx.http, "Yay! You received 200 shinies!"),
        Err(err) => msg.channel_id.say(&ctx.http, err),
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

#[command]
#[bucket = "social.profile"]
pub fn profile(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let settings = data.get::<Settings>().unwrap();
    let point_count: u32;
    let level: u32;
    let level_previous: u32;
    let level_next: u32;
    let progress: f32;

    if let Some(profile) = settings.users.fetch(msg.author.id) {
        point_count = profile.point_count;
        level = profile.get_level();
        level_previous = (level as f32 / 0.2).powf(2.0).floor() as u32;
        level_next = ((level + 1) as f32 / 0.2).powf(2.0).floor() as u32;
        progress = (point_count - level_previous) as f32 / (level_next - level_previous) as f32;
    } else {
        point_count = 0;
        level = 0;
        level_previous = 0;
        level_next = (1_f32 / 0.2).powf(2.0).floor() as u32;
        progress = 0_f32;
    }

    if let Err(why) = msg.channel_id.say(
        &ctx.http,
        format!(
            "**Level**: {level} | `{level_previous}..{point_count}..{level_next}` `[{progress}]`\n**Points**: {point_count}",
            level = level,
            level_previous = level_previous,
            level_next = level_next,
            progress = percentage(35, progress),
            point_count = point_count
        )
    ) {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}
