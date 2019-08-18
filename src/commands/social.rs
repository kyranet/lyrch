use crate::lib::core::EditableMessages;
use crate::lib::settings::Settings;
use crate::lib::util::{percentage, resolvers::resolve_user};
use crate::try_send_message_context;
use serenity::prelude::*;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

const SHINY: &'static str = "<:shiny:612364146792726539>";

group!({
    name: "social",
    options: {},
    commands: [daily, credits, profile]
});

#[command]
pub fn daily(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write();
    let settings = data.get::<Settings>().unwrap();
    match settings.users.try_daily(msg.author.id) {
        Ok(_) => try_send_message_context!(
            ctx,
            msg,
            data.get_mut::<EditableMessages>().unwrap(),
            "Yay! You received 200 {}!",
            SHINY
        ),
        Err(err) => {
            try_send_message_context!(ctx, msg, data.get_mut::<EditableMessages>().unwrap(), err)
        }
    };

    Ok(())
}

#[command]
pub fn credits(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write();
    let settings = data.get::<Settings>().unwrap();
    let amount = settings.users.retrieve_user_money_count(msg.author.id);
    try_send_message_context!(
        ctx,
        msg,
        data.get_mut::<EditableMessages>().unwrap(),
        "You have a total of {} {}",
        amount,
        SHINY
    );
    Ok(())
}

#[command]
#[usage = "[user]"]
#[bucket = "social.profile"]
pub fn profile(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let mut data = ctx.data.write();
    let settings = data.get::<Settings>().unwrap();
    let point_count: u32;
    let level: u32;
    let level_previous: u32;
    let level_next: u32;
    let progress: f32;
    let (user_name, user_id) = if let Some(user) = resolve_user(&ctx, &args) {
        let user = user.read();
        (user.name.clone(), user.id)
    } else {
        (msg.author.name.clone(), msg.author.id)
    };

    if let Some(profile) = settings.users.fetch(user_id) {
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

    try_send_message_context!(ctx, msg, data.get_mut::<EditableMessages>().unwrap(), "[ {user_name} ] **Level**: {level} | `{level_previous}..{point_count}..{level_next}` `[{progress}]`",
        level = level,
        level_previous = level_previous,
        level_next = level_next,
        progress = percentage(35, progress),
        point_count = point_count,
        user_name = user_name
    );
    Ok(())
}
