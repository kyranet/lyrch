use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};
use std::env;

#[derive(Debug, Deserialize)]
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

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
    static ref HEADER_USER_AGENT: String = format!(
        "Skyra/{type}/{version} ({codename})",
        type = "Development",
        version = "6.0.0",
        codename = "Lyrch"
    );
    static ref HEADER_TOKEN: String = format!(
        "Wolke {}",
        env::var("WEEB_SH").expect("Expected a token in the environment")
    )
    .to_owned();
}

group!({
    name: "weeb",
    options: {},
    commands: [wblush]
});

#[command]
#[only_in(guilds)]
pub fn wblush(ctx: &mut Context, msg: &Message) -> CommandResult {
    let url = format!(
        "https://api-v2.weeb.sh/images/random?type=blush&nsfw={}",
        msg.channel(&ctx).unwrap().is_nsfw()
    );

    let res: WeebSh = CLIENT
        .get(&url)
        .header(USER_AGENT, HEADER_USER_AGENT.to_owned())
        .header(AUTHORIZATION, HEADER_TOKEN.to_owned())
        .send()?
        .json()?;

    match msg.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.image(res.url);
            e.color(Colour::from_rgb(110, 136, 216));
            e.footer(|f| {
                f.text("Powered by weeb.sh");
                f
            })
        })
    }) {
        Err(error) => println!("Something went wrong: {:?}", error),
        Ok(_message) => println!("Sent blush random image"),
    };
    Ok(())
}
