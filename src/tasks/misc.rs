use serenity::{http::Http, model::prelude::*};
use std::sync::Arc;

pub fn task_reminder(http: Arc<Http>, channel: ChannelId, args: &str) {
    if let Err(error) = channel.say(&http, format!("You wanted me to remind you: {}", args)) {
        crate::error!("Could not send message: {:?}", error);
    }
}
