#[macro_export]
macro_rules! try_command {
    ($result:expr) => {
        match $result {
            Ok(value) => Ok(value),
            Err(err) => Err(serenity::framework::standard::CommandError(err.to_string())),
        }
    };
}

#[macro_export]
macro_rules! ctx_send_message {
    ($ctx:expr, $msg:expr, $redis_connection:expr, $f:expr) => {
        if let Ok(older) = $redis_connection.query(&mut redis::cmd("GET").arg($msg.id.0)) {
            let older: u64 = older;
            let older = serenity::model::prelude::MessageId(older);
            $msg.channel_id.edit_message(&$ctx, older, $f)
        } else {
            match $msg.channel_id.send_message(&$ctx, $f) {
                Ok(new_message) => {
                    $redis_connection.set_ttl($msg.id.0, new_message.id.0, 60 * 15);
                    Ok(new_message)
                }
                Err(why) => Err(why),
            }
        }
    };
}

#[macro_export]
macro_rules! try_ctx_send_message {
    ($ctx:expr, $msg:expr, $redis_connection:expr, $f:expr) => {
        $crate::try_command!($crate::ctx_send_message!($ctx, $msg, $redis_connection, $f))
    };
}

#[macro_export]
macro_rules! ctx_send_message_content {
    ($ctx:expr, $msg:expr, $redis_connection:expr, $content:expr) => {
        $crate::ctx_send_message!($ctx, $msg, $redis_connection, |e| e.content($content))
    };
    ($ctx:expr, $msg:expr, $redis_connection:expr, $content:expr, $($args:tt)*) => {
        $crate::ctx_send_message!($ctx, $msg, $redis_connection, |e| e.content(format!($content, $($args)*)))
    };
}

#[macro_export]
macro_rules! try_ctx_send_message_content {
    ($ctx:expr, $msg:expr, $redis_connection:expr, $content:expr) => {
        $crate::try_command!($crate::ctx_send_message_content!($ctx, $msg, $redis_connection, $content))
    };
    ($ctx:expr, $msg:expr, $redis_connection:expr, $content:expr, $($args:tt)*) => {
        $crate::try_command!($crate::ctx_send_message_content!($ctx, $msg, $redis_connection, $content, $($args)*))
    };
}

#[macro_export]
macro_rules! ctx_send_message_embed {
    ($ctx:expr, $msg:expr, $redis_connection:expr, $f:expr) => {
        $crate::ctx_send_message!($ctx, $msg, $redis_connection, |e| e.embed($fmt).content(""))
    };
}

#[macro_export]
macro_rules! try_ctx_send_message_embed {
    ($ctx:expr, $msg:expr, $redis_connection:expr, $f:expr) => {
        $crate::try_command!($crate::ctx_send_message_embed!(
            $ctx,
            $msg,
            $redis_connection,
            $f
        ))
    };
}

#[macro_export]
macro_rules! send_message {
    ($ctx:expr, $msg:expr, $f:expr) => {{
        let data = $ctx.data.read();
        let redis_connection = data
            .get::<crate::lib::cache::RedisConnection>()
            .expect("Expected RedisConnection in ShareMap.");

        $crate::ctx_send_message!($ctx, $msg, redis_connection, $f)
    }};
}

#[macro_export]
macro_rules! try_send_message {
    ($ctx:expr, $msg:expr, $f:expr) => {
        $crate::try_command!($crate::send_message!($ctx, $msg, $f))
    };
}

#[macro_export]
macro_rules! send_message_content {
    ($ctx:expr, $msg:expr, $content:expr) => {
        $crate::send_message!($ctx, $msg, |e| e.content($content))
    };
    ($ctx:expr, $msg:expr, $content:expr, $($args:tt)*) => {
        $crate::send_message!($ctx, $msg, |e| e.content(format!($content, $($args)*)))
    };
}

#[macro_export]
macro_rules! send_message_i18n {
    ($ctx:expr, $msg:expr, $key:ident) => {{
        let language = $crate::language_get!($ctx, $msg);
        $crate::send_message_content!($ctx, $msg, $crate::language_use!(language, $key))
    }};
    ($ctx:expr, $msg:expr, $key:ident, $($args:expr)*) => {{
        let language = $crate::language_get!($ctx, $msg);
        $crate::send_message_content!($ctx, $msg, $crate::language_use!(language, $key, $($args)*))
    }};
}

#[macro_export]
macro_rules! try_send_message_content {
    ($ctx:expr, $msg:expr, $content:expr) => {
        $crate::try_command!($crate::send_message_content!($ctx, $msg, $content))
    };
    ($ctx:expr, $msg:expr, $content:expr, $($args:tt)*) => {
        $crate::try_command!($crate::send_message_content!($ctx, $msg, $content, $($args)*))
    };
}

#[macro_export]
macro_rules! try_send_message_i18n {
    ($ctx:expr, $msg:expr, $key:ident) => {
        $crate::try_command!($crate::send_message_i18n!($ctx, $msg, $key))
    };
    ($ctx:expr, $msg:expr, $key:ident, $($args:expr)*) => {
        $crate::try_command!($crate::send_message_i18n!($ctx, $msg, $key, $($args)*))
    };
}

#[macro_export]
macro_rules! send_message_embed {
    ($ctx:expr, $msg:expr, $f:expr) => {
        $crate::send_message!($ctx, $msg, |e| e.embed($f).content(""))
    };
}

#[macro_export]
macro_rules! try_send_message_embed {
    ($ctx:expr, $msg:expr, $f:expr) => {
        $crate::try_command!($crate::send_message_embed!($ctx, $msg, $f))
    };
}

#[macro_export]
macro_rules! language_get {
    ($ctx:expr, $msg:expr) => {{
        if let Some(guild) = $msg.guild_id {
            let data = $ctx.data.read();
            let guilds = data
                .get::<crate::lib::settings::guilds::GuildSettingsHandler>()
                .expect("Expected GuildSettingsHandler in ShareMap.");
            if let Some(guild_settings) = guilds.get(guild) {
                guild_settings.language.clone()
            } else {
                "en_us".to_owned()
            }
        } else {
            "en_us".to_owned()
        }
    }};
}

#[macro_export]
macro_rules! language_use {
    ($language:expr, $key:ident) => {
        crate::i18n::OUTPUT.get(&$language).unwrap().$key()
    };
    ($language:expr, $key:ident, $($args:expr)*) => {
        crate::i18n::OUTPUT.get(&$language).unwrap().$key($(&$args)*)
    };
}
