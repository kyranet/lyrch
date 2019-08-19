use crate::try_send_message;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::{
        channel::{Message, PermissionOverwrite, PermissionOverwriteType},
        Permissions,
    },
};

const LOCKDOWN_PERMISSION: Permissions = Permissions::SEND_MESSAGES;

group!({
    name: "moderation",
    options: {},
    commands: [lockdown]
});

#[command]
#[only_in(guilds)]
pub fn lockdown(ctx: &mut Context, msg: &Message) -> CommandResult {
    // TODO: Check channel perms to see if the channel already locked?
    let channel_is_locked = false;

    let overwrite = if channel_is_locked {
        PermissionOverwrite {
            allow: LOCKDOWN_PERMISSION,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(RoleId(msg.guild_id.unwrap().0)),
        }
    } else {
        PermissionOverwrite {
            deny: LOCKDOWN_PERMISSION,
            allow: Permissions::empty(),
            kind: PermissionOverwriteType::Role(RoleId(msg.guild_id.unwrap().0)),
        }
    };

    let channel_name = msg.channel_id.name(&ctx.cache).unwrap();
    let patience = if channel_is_locked {
        format!("Locking the channel {}...", channel_name)
    } else {
        format!("Unlocking the channel {}...", channel_name)
    };

    try_send_message!(ctx, msg, patience);

    // assuming the cache has been unlocked
    let channel = &ctx
        .cache
        .read()
        .guild_channel(msg.channel_id.0)
        .ok_or(ModelError::ItemMissing)?;

    channel.read().create_permission(&ctx.http, &overwrite)?;

    let response = if channel_is_locked {
        format!(
            "The lockdown for the channel {} has been released.",
            channel_name
        )
    } else {
        format!("The channel {} is now locked.", channel_name)
    };
    try_send_message!(ctx, msg, response);
    Ok(())
}
