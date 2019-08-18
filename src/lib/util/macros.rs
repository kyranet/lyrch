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
macro_rules! send_message_context {
    ($ctx:expr, $msg:expr, $redis_connection:expr, $fmt:expr) => {
        if let Ok(older) = $redis_connection.query(&mut redis::cmd("GET").arg($msg.id.0)) {
            let older: u64 = older;
            let older = serenity::model::prelude::MessageId(older);
            $msg.channel_id
                .edit_message(&$ctx, older, |e| e.content($fmt))
        } else {
            match $msg.channel_id.say(&$ctx, $fmt) {
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
macro_rules! try_send_message_context {
    ($ctx:expr, $msg:expr, $redis_connection:expr, $content:expr) => {
        crate::try_command!(crate::send_message_context!(
            $ctx,
            $msg,
            $redis_connection,
            $content
        ))?
    };
    ($ctx:expr, $msg:expr, $redis_connection:expr, $content:expr, $($args:tt)+) => {
        try_send_message_context!(
            $ctx,
            $msg,
            $redis_connection,
            format!($content, $($args)+)
        )
    };
}

#[macro_export]
macro_rules! send_message {
    ($ctx:expr, $msg:expr, $content:expr) => {
        {
            let mut data = $ctx.data.write();
            let redis_connection = data
                .get_mut::<crate::lib::cache::RedisConnection>()
                .expect("Expected RedisConnection in ShareMap.");

            crate::send_message_context!($ctx, $msg, redis_connection, $content)
        }
    };
    ($ctx:expr, $msg:expr, $content:expr, $($args:tt)+) => {
        send_message!($ctx, $msg, format!($content, $($args)*))
    };
}

#[macro_export]
macro_rules! try_send_message {
    ($ctx:expr, $msg:expr, $content:expr) => {
        crate::try_command!(crate::send_message!($ctx, $msg, $content))?
    };
    ($ctx:expr, $msg:expr, $content:expr, $($args:tt)+) => {
        crate::try_command!(crate::send_message!($ctx, $msg, format!($content, $($args)+)))?
    };
}
