use crate::lib::core::ThreadPoolContainer;
use crate::lib::framework::LyrchFramework;
use crate::lib::settings::guilds::GuildSettingsHandler;
use crate::lib::settings::SettingsHandler;
use crate::serenity::framework::Framework;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::sync::Arc;
use threadpool::ThreadPool;

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.cache.write().settings_mut().max_messages(50);
    }

    fn message_update(
        &self,
        ctx: Context,
        _: Option<Message>,
        new_message: Option<Message>,
        _: MessageUpdateEvent,
    ) {
        if let Some(message) = new_message {
            let mut framework: LyrchFramework;
            let tp: Arc<Mutex<ThreadPool>>;
            {
                let data = ctx.data.read();
                framework = data.get::<LyrchFramework>().unwrap().clone();
                tp = data.get::<ThreadPoolContainer>().unwrap().clone();
            }
            framework.dispatch(ctx, message, &tp.lock());
        }
    }

    fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        let mut data = ctx.data.write();
        let settings = data.get_mut::<GuildSettingsHandler>().unwrap();
        let guild_settings = settings.fetch(guild.id);
        settings.add(guild_settings);
    }

    fn guild_delete(
        &self,
        ctx: Context,
        incomplete: PartialGuild,
        _full: Option<Arc<RwLock<Guild>>>,
    ) {
        let mut data = ctx.data.write();
        let settings = data.get_mut::<GuildSettingsHandler>().unwrap();
        settings.remove(incomplete.id);
    }
}
