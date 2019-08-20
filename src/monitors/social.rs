use crate::lib::settings::users::UserSettingsHandler;
use crate::lib::settings::SettingsHandler;
use crate::lib::util::bucket::{Bucket, Ratelimit};
use lazy_static;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;

pub fn social_points(ctx: &mut Context, msg: &Message) -> bool {
    lazy_static! {
        static ref SOCIAL_POINTS_BUCKET: Mutex<Bucket> = Mutex::new(Bucket {
            ratelimit: Ratelimit {
                delay: 60,
                limit: None
            },
            users: HashMap::default()
        });
    }

    if SOCIAL_POINTS_BUCKET.lock().take(msg.author.id.0) == 0 {
        let data = ctx.data.read();
        let settings = data.get::<UserSettingsHandler>().unwrap();
        if let Err(err) = settings.update_increase(msg.author.id, "point_count", &5) {
            println!("Failed to update points for {}: {}", msg.author.id, err);
        } else {
            println!("Updated points for {}", msg.author.id)
        }
    }

    true
}
