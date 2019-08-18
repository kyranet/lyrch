#[macro_export]
macro_rules! send_message {
    ($ctx:expr, $msg:expr, $fmt:expr) => {
        {
            use crate::lib::core::EditableMessages;

            let mut data = $ctx.data.write();
            let editable_messages = data
                .get_mut::<EditableMessages>()
                .expect("Expected EditableMessages in ShareMap.");

            if let Some(older) = editable_messages.get(&$msg.id) {
                $msg.channel_id.edit_message(&$ctx, older, |e| {
                    e.content($fmt)
                })
            } else {
                match $msg.channel_id.say(&$ctx, $fmt) {
                    Ok(new_message) => {
                        editable_messages.insert($msg.id, new_message.id);
                        Ok(new_message)
                    },
                    Err(why) => Err(why)
                }
            }
        }
    };
    ($ctx:expr, $msg:expr, $fmt:expr, $($args:tt)+) => {
        send_message!($ctx, $msg, format!($fmt, $($args)*))
    };
}
