use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(ping, help, kiss)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
	msg.reply(ctx, "pong :3").await?;

	Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
	msg.reply(ctx,
		"commands: help, kiss, ping\n\
		:3").await?;

	Ok(())
}

#[command]
async fn kiss(ctx: &Context, msg: &Message) -> CommandResult {
	let mentions = &msg.mentions;

	if mentions.len() == 0 {
		msg.reply(ctx, "please tag the person u wanna kiss :3").await?;
		return Ok(());
	} else if mentions.len() > 1 {
		msg.reply(ctx, "polyamory not implimented yet :<").await?;
		return Ok(());
	}

	let replymsg = format!(
		"{} kisses {} >w<\n\
		\\*yuri gif goes here\\*", msg.author.name, mentions[0].name);

	msg.reply(ctx, replymsg).await?;

	Ok(())
}

#[tokio::main]
async fn main() {
	let framework = StandardFramework::new()
		.configure(|c| c.prefix("nya ")) // set the bot's prefix to "~"
		.group(&GENERAL_GROUP);

	// Login with a bot token from the environment
	let token = env::var("DISCORD_TOKEN").expect("token");
	let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
	let mut client = Client::builder(token, intents)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("Error creating client");

	// start listening for events by starting a single shard
	if let Err(why) = client.start().await {
		println!("An error occurred while running the client: {:?}", why);
	}
}
