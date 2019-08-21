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
        "{name}/{type}/{version} ({codename})",
        name = env::var("CLIENT_NAME").expect("Expected the `CLIENT_NAME` variable to be set in `.env`."),
        type = env::var("KIND").expect("Expected the `KIND` variable to be set in `.env`."),
        version = env::var("VERSION").expect("Expected the `VERSION` variable to be set in `.env`."),
        codename = env::var("CODENAME").expect("Expected the `CODENAME` variable to be set in `.env`.")
    );
    static ref HEADER_TOKEN: String = format!(
        "Wolke {}",
        env::var("WEEB_SH").expect("Expected a token in the environment")
    );
}

macro_rules! create_weeb_command {
    ($($command:ident;)*) => {
        group!({
            name: "weeb",
            options: {
                prefix: "weeb",
            },
            commands: [$(
                $command,
            )*]
        });

        $(
            #[command]
            #[only_in(guilds)]
            pub fn $command(ctx: &mut Context, msg: &Message) -> CommandResult {
                let url = format!(
                    concat!("https://api-v2.weeb.sh/images/random?type=", stringify!($command), "&nsfw={}"),
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
                    Err(error) => crate::wtf!("Something went wrong: {:?}", error),
                    Ok(_message) => crate::verbose!("Sent blush random image"),
                };
                Ok(())
            }
        )*
    };
}

create_weeb_command! {
    animal_cat;
    animal_dog;
    awoo;
    baka;
    bang;
    banghead;
    bite;
    blush;
    clagwimoth;
    cry;
    cuddle;
    dab;
    dance;
    delet_this;
    deredere;
    discord_memes;
    greet;
    handholding;
    highfive;
    hug;
    initial_d;
    insult;
    jojo;
    kemonomimi;
    kiss;
    lewd;
    lick;
    megumin;
    nani;
    neko;
    nom;
    owo;
    pat;
    poi;
    poke;
    pout;
    punch;
    rem;
    shrug;
    slap;
    sleepy;
    smile;
    smug;
    stare;
    sumfuk;
    teehee;
    thinking;
    thumbsup;
    tickle;
    trap;
    triggered;
    wag;
    waifu_insult;
    wasted;
}
