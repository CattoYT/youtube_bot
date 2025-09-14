use std::fs;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use songbird::SerenityInit;
struct Handler;

mod slash_commands;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {

    let token =
        fs::read_to_string("token.txt")
            .expect("Failed to read token file")
            .trim()
            .to_string();

    if token.is_empty() {
        println!("Token file is empty. Please provide a valid token.");
        
        std::io::stdin().read_line(&mut String::new()).unwrap();
        std::process::exit(1);
    }

        let intents = GatewayIntents::all();


        let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .register_songbird()
        .framework(slash_commands::create_framework().await.unwrap())
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

}
