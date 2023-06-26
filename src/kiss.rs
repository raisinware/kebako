use crate::YuriGifCmd;

pub struct Kiss();

impl YuriGifCmd for Kiss {
	const GIFS: &'static [&'static str] = &[
		"https://tenor.com/view/poggers-pog-pogchamp-lesbian-anime-love-gif-20352902",
		"https://cdn.weeb.sh/images/ByTBhp_vZ.gif",
		"https://cdn.weeb.sh/images/SJSr3TOv-.gif",
		"https://cdn.weeb.sh/images/S1y-4l5Jf.gif",
		"https://cdn.weeb.sh/images/ByiMna_vb.gif",
		"https://cdn.weeb.sh/images/ryceu6V0W.gif",
		"https://tenor.com/view/anime-kiss-yuri-citrus-gif-13855137",
		"https://tenor.com/view/yuri-anime-kiss-gif-5999933",
		"https://tenor.com/view/yuri-gif-24238090",
		"https://tenor.com/view/kabedon-yohariko-yoshiko-yohane-love-live-gif-20913824",
		"https://tenor.com/view/kiss-yuri-anime-sakura-sakura-kiss-gif-15111552",
		"https://tenor.com/view/sono-hanabira-ni-kuchizuke-wo-anime-yuri-kiss-gif-19434907",
		"https://tenor.com/view/bloom-into-you-yagate-kimi-ni-naru-yuri-kiss-gif-21637575",
		"https://tenor.com/view/sakura-trick-kiss-yuri-gif-11487318",
	];

	fn reply_msg_none(_: &String) -> String {
		"please tag the person u wanna kiss :3".to_string()
	}

	fn reply_msg_single(author: &String, target: &String) -> String {
		format!("{} kisses {} >w<\n{}", author, target, Self::get_rand())
	}

	fn reply_msg_multi(_: &String, _: Vec<&String>) -> String {
		"polyamory not implimented yet :<".to_string()
	}
}
