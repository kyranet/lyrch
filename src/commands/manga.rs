use crate::try_send_message_content;
use reqwest;
use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

group!({
	name: "manga",
	options: {},
	commands: [manga]
});

#[command]
#[only_in(guilds)]
pub fn manga(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get the name of the manga they want to use or set it to 0 if none provided
    let name = args.single().unwrap_or(String::from("---"));
    // If there was no name provided cancel out
    if name == "---" {
        try_send_message_content!(ctx, msg, "You did not provide a manga name. Please make sure to replace all `spaces` inside a name with `-`. For example: `One Piece` should be done as `!manga one-piece`")?;
        return Ok(());
    };
    // Use the chapter they provided or default to first chapter
    let chapter = args.single().unwrap_or(String::from("1"));
    // Use the page they provided or default to page 1
    let page = args.single::<u32>().unwrap_or(1);
    // Create the url with the name chapter and page
    let url = format!("https://www.mangapanda.com/{}/{}/{}", name, chapter, page);
    // Fetch the url and convert it to text
    let res = reqwest::get(&url)?.text()?;
    // Find the index of the id="img"
    let img_index = res.find("id=\"img\"").unwrap_or(0);
    // Cut out everything before that img_index
    let img_string = &res[img_index..];
    // Using the new shorter string find the first src= to get the image url
    let src_index = img_string.find("src=").unwrap_or(0);

    let start = &img_string[src_index + 5..];

    let end_index = start.find("\" alt=").unwrap_or(res.len());
    // Send the embed with the image
    let response = crate::send_message_embed!(ctx, msg, |e| {
        e.title(format!("One Piece: Chapter {} Page {}", chapter, page));
        e.image(&start[..end_index]);
        e.timestamp(&msg.timestamp);
        e.footer(|f| {
            f.text("Powered by MangaPanda!");
            f
        });
        e.color(Colour::from_rgb(110, 136, 216))
    })?;
    // If the page is NOT the first page then add a back reaction
    if page > 1 {
        match response.react(&ctx, "⬅") {
            Ok(res) => res,
            Err(error) => println!("Unable to react on manga command: {:?}", error),
        };
    }

    match response.react(&ctx, "➡") {
        Ok(res) => res,
        Err(error) => println!("Unable to react on manga command: {:?}", error),
    };

    Ok(())
}
