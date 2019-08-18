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
    ($ctx:expr, $msg:expr, $editable_messages:expr, $fmt:expr) => {
        if let Some(older) = $editable_messages.get(&$msg.id) {
            $msg.channel_id
                .edit_message(&$ctx, older, |e| e.content($fmt))
        } else {
            match $msg.channel_id.say(&$ctx, $fmt) {
                Ok(new_message) => {
                    $editable_messages.insert($msg.id, new_message.id);
                    Ok(new_message)
                }
                Err(why) => Err(why),
            }
        }
    };
}

#[macro_export]
macro_rules! try_send_message_context {
    ($ctx:expr, $msg:expr, $editable_messages:expr, $content:expr) => {
        crate::try_command!(crate::send_message_context!(
            $ctx,
            $msg,
            $editable_messages,
            $content
        ))?
    };
    ($ctx:expr, $msg:expr, $editable_messages:expr, $content:expr, $($args:tt)+) => {
        try_send_message_context!(
            $ctx,
            $msg,
            $editable_messages,
            format!($content, $($args)+)
        )
    };
}

#[macro_export]
macro_rules! send_message {
    ($ctx:expr, $msg:expr, $content:expr) => {
        {
            let mut data = $ctx.data.write();
            let editable_messages = data
                .get_mut::<crate::lib::core::EditableMessages>()
                .expect("Expected EditableMessages in ShareMap.");

            crate::send_message_context!($ctx, $msg, editable_messages, $content)
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
