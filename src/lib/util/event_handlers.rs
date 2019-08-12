use crate::lib::settings::Settings;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::sync::Arc;

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        let mut data = ctx.data.write();
        let settings = data.get_mut::<Settings>().unwrap();
        if let Some(guild_settings) = settings.guilds.fetch(guild.id) {
            settings.guilds.add(guild_settings);
        }
    }

    fn guild_delete(
        &self,
        ctx: Context,
        incomplete: PartialGuild,
        _full: Option<Arc<RwLock<Guild>>>,
    ) {
        let mut data = ctx.data.write();
        let settings = data.get_mut::<Settings>().unwrap();
        settings.guilds.remove(incomplete.id);
    }
}
