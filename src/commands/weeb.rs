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
use serenity::serde::{Deserialize, Serialize};

const VERSION: &str = "5.1.0 Nerom";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct WeebSh {
    status: u32,
    id: String,
    type: String,
    baseType: String,
    nsfw: bool,
    fileType: String,
    mimeType: String,
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
    let res = CLIENT.get(&url)
        .header(USER_AGENT, user_agent)
        .header(AUTHORIZATION, token)
        .send()?
        .json()?;

    print!("please stop breaking");
    println!("my res {:?}", res);
    Ok(())
}
