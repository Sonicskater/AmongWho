use serenity::async_trait;
use serenity::collector;
use serenity::client::{Client, Context, EventHandler,};
use serenity::model::channel::{Message, ReactionType};
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
use serenity::utils::MessageBuilder;
use serenity::model::prelude::User;

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
    let reply = format!("Pong");
    msg.reply(ctx, reply).await?;
    Ok(())
}

#[command]
async fn lfg(ctx: &Context, msg: &Message) -> CommandResult {
    msg.delete(ctx).await?;

    let content = MessageBuilder::new()
        .push("@here")
        .mention(&msg.author)
        .push(" is looking for players!")
        .push("\nReact to this message with 👍 to join this game.").build();

    let posting = msg.reply(ctx, content).await?;

    posting.react(ctx, ReactionType::from('👍')).await?;

    loop {
        if let Some(reaction) = &posting.await_reaction(&ctx).timeout(Duration::from_secs(2*15)).await {

            let users : Vec<User> = reaction.as_inner_ref().users(ctx,ReactionType::from('👍'),Some(10), Some(posting.author.id)).await?;

            if users.len() >= 6 {
                let mut call = MessageBuilder::new();
                call.push("Hey, enough players have signed up!");
                for user in users{
                    call.mention(&user);
                }
                call.push(" will be playing today.");
                call.push("\nThe posting will now be closed.");
                posting.delete(ctx).await?;

                break;
            }

        } else {
            msg.reply(ctx,"Not enough people have joined. Maybe try again later?").await?;
            break;
        }
    }
    Ok(())
}