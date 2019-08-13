pub mod social;

use serenity::{model::prelude::Message, prelude::Context};

pub fn run(ctx: &mut Context, msg: &Message) -> bool {
    social::social_points(ctx, msg)
}
