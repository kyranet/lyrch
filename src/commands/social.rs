use crate::lib::cache::RedisConnection;
use crate::lib::settings::users::UserSettingsHandler;
use crate::lib::settings::SettingsHandler;
use crate::lib::util::{percentage, resolvers::resolve_user};
use crate::try_ctx_send_message_content;
use serenity::prelude::*;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

const SHINY: &str = "<:shiny:612364146792726539>";

group!({
    name: "social",
    options: {},
    commands: [daily, credits, profile]
});

#[command]
pub fn daily(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let settings = data.get::<UserSettingsHandler>().unwrap();
    match settings.try_daily(msg.author.id) {
        Ok(_) => try_ctx_send_message_content!(
            ctx,
            msg,
            data.get::<RedisConnection>().unwrap(),
            "Yay! You received 200 {}!",
            SHINY
        )?,
        Err(err) => {
            try_ctx_send_message_content!(ctx, msg, data.get::<RedisConnection>().unwrap(), err)?
        }
    };

    Ok(())
}

#[command]
pub fn credits(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let settings = data.get::<UserSettingsHandler>().unwrap();
    let amount = settings.retrieve_user_money_count(msg.author.id);
    try_ctx_send_message_content!(
        ctx,
        msg,
        data.get::<RedisConnection>().unwrap(),
        "You have a total of {} {}",
        amount,
        SHINY
    )?;
    Ok(())
}

#[command]
#[usage = "[user]"]
#[bucket = "social.profile"]
pub fn profile(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read();
    let settings = data.get::<UserSettingsHandler>().unwrap();

    let (user_name, user_id) = if let Some(user) = resolve_user(&ctx, &args) {
        let user = user.read();
        (user.name.clone(), user.id)
    } else {
        (msg.author.name.clone(), msg.author.id)
    };

    let profile = settings.fetch(user_id);
    let point_count = profile.point_count;
    let level = profile.get_level();
    let level_previous = (level as f32 / 0.2).powf(2.0).floor() as i32;
    let level_next = ((level + 1) as f32 / 0.2).powf(2.0).floor() as i32;
    let progress = (point_count - level_previous) as f32 / (level_next - level_previous) as f32;

    try_ctx_send_message_content!(ctx, msg, data.get::<RedisConnection>().unwrap(),
        "[ {user_name} ] **Level**: {level} | `{level_previous}..{point_count}..{level_next}` `[{progress}]`",
        level = level,
        level_previous = level_previous,
        level_next = level_next,
        progress = percentage(35, progress),
        point_count = point_count,
        user_name = user_name
    )?;
    Ok(())
}
