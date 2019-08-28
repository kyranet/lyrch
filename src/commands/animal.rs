use reqwest;
use serde::{Deserialize, Serialize};
use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

group!({
	name: "animal",
	options: {},
	commands: [catfact, kitty, dog, fox, shibe]
});

#[derive(Debug, Serialize, Deserialize)]
pub struct CatFactData {
    pub fact: String,
    pub length: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DogData {
    pub message: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KittyData {
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FoxData {
    pub image: String,
    pub link: String,
}

#[command]
#[only_in(guilds)]
pub fn catfact(ctx: &mut Context, msg: &Message) -> CommandResult {
    lazy_static! {
        static ref CLIENT: reqwest::Client = reqwest::Client::new();
    }

    let res: CatFactData = CLIENT.get("https://catfact.ninja/fact").send()?.json()?;

    crate::try_send_message_embed!(ctx, msg, |e| {
        e.title("Cat Fact");
        e.description(res.fact);
        e.color(Colour::from_rgb(110, 136, 216))
    })?;
    Ok(())
}

#[command]
#[only_in(guilds)]
pub fn dog(ctx: &mut Context, msg: &Message) -> CommandResult {
    lazy_static! {
        static ref CLIENT: reqwest::Client = reqwest::Client::new();
    }

    let res: DogData = CLIENT
        .get("https://dog.ceo/api/breeds/image/random")
        .send()?
        .json()?;

    crate::try_send_message_embed!(ctx, msg, |e| {
        e.image(res.message);
        e.timestamp(&msg.timestamp);
        e.color(Colour::from_rgb(110, 136, 216))
    })?;
    Ok(())
}

#[command]
#[only_in(guilds)]
pub fn kitty(ctx: &mut Context, msg: &Message) -> CommandResult {
    lazy_static! {
        static ref CLIENT: reqwest::Client = reqwest::Client::new();
    }

    let res: KittyData = CLIENT.get("https://aws.random.cat/meow").send()?.json()?;

    crate::try_send_message_embed!(ctx, msg, |e| {
        e.image(res.file);
        e.timestamp(&msg.timestamp);
        e.color(Colour::from_rgb(110, 136, 216))
    })?;
    Ok(())
}

#[command]
#[only_in(guilds)]
pub fn fox(ctx: &mut Context, msg: &Message) -> CommandResult {
    lazy_static! {
        static ref CLIENT: reqwest::Client = reqwest::Client::new();
    }

    let res: FoxData = CLIENT.get("https://randomfox.ca/floof").send()?.json()?;

    crate::try_send_message_embed!(ctx, msg, |e| {
        e.image(res.image);
        e.timestamp(&msg.timestamp);
        e.color(Colour::from_rgb(110, 136, 216))
    })?;
    Ok(())
}

#[command]
#[only_in(guilds)]
pub fn shibe(ctx: &mut Context, msg: &Message) -> CommandResult {
    lazy_static! {
        static ref CLIENT: reqwest::Client = reqwest::Client::new();
    }

    let res: Vec<String> = CLIENT
        .get("http://shibe.online/api/shibes?count=1")
        .send()?
        .json()?;

    crate::try_send_message_embed!(ctx, msg, |e| {
        e.image(&res[0]);
        e.timestamp(&msg.timestamp);
        e.color(Colour::from_rgb(110, 136, 216))
    })?;
    Ok(())
}
