use std::env;

use nyabot::YuriGifCmd;
use nyabot::kiss::Kiss;
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
	Kiss::cmd_impl(ctx, msg).await
}

#[tokio::main]
async fn main() {
	let framework = StandardFramework::new()
		.configure(|c| c.prefix("nya ")) // set the bot's prefix to "nya "
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
