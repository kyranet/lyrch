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
pub fn catfact(ctx: &mut Context, msg: &Message) -> CommandResult {
    let res: CatFactData = reqwest::get("https://catfact.ninja/fact")?.json()?;

    crate::try_send_message_embed!(ctx, msg, |e| {
        e.title("Cat Fact");
        e.description(res.fact);
        e.color(Colour::from_rgb(110, 136, 216))
    })?;
    Ok(())
}

#[command]
pub fn dog(ctx: &mut Context, msg: &Message) -> CommandResult {
    let res: DogData = reqwest::get("https://dog.ceo/api/breeds/image/random")?.json()?;

    crate::try_send_message_embed!(ctx, msg, |e| {
        e.image(res.message);
        e.timestamp(&msg.timestamp);
        e.color(Colour::from_rgb(110, 136, 216))
    })?;
    Ok(())
}

#[command]
pub fn kitty(ctx: &mut Context, msg: &Message) -> CommandResult {
    let res: KittyData = reqwest::get("https://aws.random.cat/meow")?.json()?;

    crate::try_send_message_embed!(ctx, msg, |e| {
        e.image(res.file);
        e.timestamp(&msg.timestamp);
        e.color(Colour::from_rgb(110, 136, 216))
    })?;
    Ok(())
}

#[command]
pub fn fox(ctx: &mut Context, msg: &Message) -> CommandResult {
    let res: FoxData = reqwest::get("https://randomfox.ca/floof")?.json()?;

    crate::try_send_message_embed!(ctx, msg, |e| {
        e.image(res.image);
        e.timestamp(&msg.timestamp);
        e.color(Colour::from_rgb(110, 136, 216))
    })?;
    Ok(())
}

#[command]
pub fn shibe(ctx: &mut Context, msg: &Message) -> CommandResult {
    let res: Vec<String> = reqwest::get("http://shibe.online/api/shibes?count=1")?.json()?;

    crate::try_send_message_embed!(ctx, msg, |e| {
        e.image(&res[0]);
        e.timestamp(&msg.timestamp);
        e.color(Colour::from_rgb(110, 136, 216))
    })?;
    Ok(())
}
