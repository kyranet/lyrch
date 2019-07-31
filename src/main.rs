extern crate dotenv;
extern crate serenity;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            let response = msg.channel_id.say(&context, "Ping...");
            if let Ok(reply) = response {
                let latency = (reply.timestamp - msg.timestamp).num_milliseconds();
                msg.channel_id
                    .edit_message(&context, reply, |e| {
                        e.content(format!("Pong! Took: {}ms!", latency))
                    })
                    .ok();
            }
        } else if msg.content == "!messageme" {
            // If the `utils`-feature is enabled, then model structs will
            // have a lot of useful methods implemented, to avoid using an
            // often otherwise bulky Context, or even much lower-level `rest`
            // method.
            //
            // In this case, you can direct message a User directly by simply
            // calling a method on its instance, with the content of the
            // message.
            let dm = msg.author.dm(&context, |m| {
                m.content("Hello!");
                m
            });

            if let Err(why) = dm {
                println!("Error when direct messaging user: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Run dotenv first.
    dotenv::dotenv().unwrap();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
