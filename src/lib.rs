use async_trait::async_trait;
use rand::{thread_rng, Rng};
use serenity::{model::{user::User, prelude::Message}, prelude::Context, framework::standard::CommandResult};

pub mod kiss;

#[async_trait]
pub trait YuriGifCmd {
	const GIFS: &'static [&'static str];

	fn get_rand() -> &'static str {
		let mut rng = thread_rng();
		let range = rng.gen_range(0..Self::GIFS.len());

		Self::GIFS[range]
	}

	async fn cmd_impl(ctx: &Context, msg: &Message) -> CommandResult {
		let author = &(msg.author.name);
		let targets = get_all_mentioned_users(&(msg.mentions));

		if targets.len() == 0 {
			msg.reply(ctx, Self::reply_msg_none(author)).await?;
			return Ok(());
		} else if targets.len() > 1 {
			msg.reply(ctx, Self::reply_msg_multi(author, targets)).await?;
			return Ok(());
		}

		msg.reply(ctx, Self::reply_msg_single(author, targets[0])).await?;
		Ok(())
	}

	fn reply_msg_none(author: &String) -> String;

	fn reply_msg_single(author: &String, target: &String) -> String;

	fn reply_msg_multi(author: &String, targets: Vec<&String>) -> String;
}

pub fn get_all_mentioned_users(mentions: &Vec<User>) -> Vec<&String> {
	let mut array:Vec<&String> = vec![];

	for user in mentions {
		array.push(&user.name);
	}

	array
}
