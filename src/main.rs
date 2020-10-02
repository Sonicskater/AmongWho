use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler,};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::env;
use tokio::time::{delay_for, Instant};
use serenity::static_assertions::_core::time::Duration;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token not found, please set env variable DISCORD_TOKEN");
    //let token = "NzYxNDE3MTQ2MjMxODE2MjQz.X3aS-g.vNStpBueY96hwxFt6wULNjGoA1s";
    let mut client = Client::new(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {


    println!("Ponged {}",msg.author.name);

    for i in 0..10 {
        let reply = format!("Pong {}", i);
        msg.reply(ctx, reply).await?;
        for r in &msg.reactions {
            let report = format!("Reaction {} x {}",r.reaction_type,r.count);
            msg.reply(ctx, report).await?;
        }
        delay_for(Duration::from_millis(10000)).await;
    }

    Ok(())
}