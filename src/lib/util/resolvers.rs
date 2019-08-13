use serenity::{framework::standard::Args, model::prelude::*, prelude::*, utils::parse_username};
use std::sync::Arc;

pub fn resolve_user(ctx: &Context, args: &Args) -> Option<Arc<RwLock<User>>> {
    if args.is_empty() {
        None
    } else if let Ok(parsed_user_id) = args.parse::<u64>() {
        resolve_user_id(ctx, parsed_user_id)
    } else if let Some(parsed_user_id) = parse_username(args.current().unwrap()) {
        resolve_user_id(ctx, parsed_user_id)
    } else {
        None
    }
}

fn resolve_user_id(ctx: &Context, raw_user_id: u64) -> Option<Arc<RwLock<User>>> {
    let user_id = UserId(raw_user_id);
    let cache = ctx.cache.read();
    if let Some(user) = cache.users.get(&user_id) {
        Some(user.clone())
    } else if let Ok(user) = ctx.http.get_user(raw_user_id) {
        Some(Arc::new(RwLock::new(user)))
    } else {
        None
    }
}
