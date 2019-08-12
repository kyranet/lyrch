use lazy_static;
use regex::Regex;
use serenity::{framework::standard::Args, model::prelude::*, prelude::*};

pub fn resolve_user(ctx: &Context, args: &Args) -> Option<UserId> {
    lazy_static! {
        static ref REGEX_USER_ID: Regex = Regex::new(r"<@!?(\d{17,18})>").unwrap();
    }
    if args.is_empty() {
        None
    } else if let Ok(parsed_user_id) = args.parse::<u64>() {
        resolve_user_id(ctx, parsed_user_id)
    } else if let Some(captures) = REGEX_USER_ID.captures(args.current().unwrap()) {
        if let Ok(parsed_user_id) = captures.get(1).unwrap().as_str().parse::<u64>() {
            resolve_user_id(ctx, parsed_user_id)
        } else {
            None
        }
    } else {
        None
    }
}

fn resolve_user_id(ctx: &Context, raw_user_id: u64) -> Option<UserId> {
    let user_id = UserId(raw_user_id);
    let cache = ctx.cache.read();
    if cache.users.get(&user_id).is_some() || ctx.http.get_user(raw_user_id).is_ok() {
        Some(user_id)
    } else {
        None
    }
}
