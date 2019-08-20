use serenity::prelude::*;
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};
use reqwest::header::{USER_AGENT, AUTHORIZATION};
use std::env;
use serde::{Deserialize};

const VERSION: &str = "5.1.0 Nerom";

#[derive(Debug)]
#[derive(Deserialize)]
struct WeebSh {
    status: u32,
    id: String,
    r#type: String,
    #[serde(rename(deserialize = "baseType"))]
    base_type: String,
    nsfw: bool,
    #[serde(rename(deserialize = "fileType"))]
    file_type: String,
    #[serde(rename(deserialize = "mimeType"))]
    mime_type: String,
    account: String,
    hidden: bool,
    tags: Vec<String>,
    url: String,
}

group!({
    name: "weeb",
    options: {},
    commands: [wblush]
});

#[command]
#[only_in(guilds)]
pub fn wblush(ctx: &mut Context, msg: &Message) -> CommandResult {
    lazy_static! {
        static ref CLIENT: reqwest::Client = reqwest::Client::new();
    }

    let url = format!(
        "https://api-v2.weeb.sh/images/random?type=blush&nsfw={}",
        msg.channel(&ctx).unwrap().is_nsfw()
    );

    let token: String = format!("Wolke {}", env::var("WEEB_SH").expect("Expected a token in the environment"));
    let user_agent = format!("Skyra/{}", VERSION);
    let res: WeebSh = CLIENT.get(&url)
        .header(USER_AGENT, user_agent)
        .header(AUTHORIZATION, token)
        .send()?
        .json()?;

    match msg.channel_id.say(&ctx.http, &res.url) {
        Err(error) => println!("Something went wrong: {:?}", error),
        Ok(_message) => println!("Sent blush random image"),
    };
    Ok(())
}
