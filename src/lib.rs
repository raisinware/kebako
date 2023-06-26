use rand::{thread_rng, Rng};

pub mod kiss;

pub trait YuriGif {
	const GIFS: &'static [&'static str];

	fn get_rand() -> &'static str {
		let mut rng = thread_rng();
		let range = rng.gen_range(0..Self::GIFS.len());

		Self::GIFS[range]
	}

	fn reply_msg_none(author: &String) -> String;

	fn reply_msg_single(author: &String, recipient: &String) -> String;

	fn reply_msg_multi(author: &String, recipients: Vec<String>) -> String;
}
