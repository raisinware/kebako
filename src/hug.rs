use crate::YuriGifCmd;

pub struct Hug();

impl YuriGifCmd for Hug {
	const CMD_NAME: &'static str = "hug";

	const GIFS: &'static [&'static str] = &[
		"https://tenor.com/view/run-hug-hug-yuri-anime-girl-gif-25253471",
		"https://tenor.com/view/love-hug-anime-gif-20942848",
		"https://tenor.com/view/girls-hugging-yuri-hug-anime-girls-502198-gif-19599352",
		"https://tenor.com/view/hug-cuddle-anime-cute-anime-hug-gif-18960633",
		"https://tenor.com/view/hug-gif-20943092",
		"https://tenor.com/view/citrus-harumi-yuzu-hug-anime-gif-18171387",
		"https://tenor.com/view/adachi-to-shimamura-adachi-sakura-shimamura-hougetsu-hug-yuri-gif-26001019",
		"https://tenor.com/view/anime-cry-hug-gif-18191007",
		"https://tenor.com/view/anime-hug-wholesome-cute-gif-15628247",
		"https://tenor.com/view/anime-hug-hug-hugs-anime-girl-anime-girl-hug-gif-16787485",
		"https://tenor.com/view/abra%C3%A7o-bff-hug-gif-14903946",
		"https://tenor.com/view/anime-hug-love-cuddle-clingy-gif-17235256",
		"https://tenor.com/view/anime-happy-girl-hug-shocked-gif-15788552",
		"https://tenor.com/view/hugs-best-friends-gif-24860485",
		"https://tenor.com/view/lycoris-recoil-spinning-hug-gif-26347742",
		"https://tenor.com/view/hololive-watame-hug-gif-18040611",
		"https://tenor.com/view/ocha-pmmm-madoka-kaname-homura-akemi-madohomu-gif-23105013",
		"https://tenor.com/view/idk-what-anime-this-is-from-but-its-anime-girls-hugging-yuri-anime-girl-hug-girl-hug-gif-19599316",
		"https://tenor.com/view/girl-anime-anime-hug-hugs-hug-gif-25253751",
		"https://tenor.com/view/kanamari-hug-yuri-love-live-love-live-sunshine-gif-22783100",
		"https://tenor.com/view/anime-hug-girl-girlxgirl-cute-gif-22306001"
	];

	fn reply_msg_single(author: &String, target: &String) -> String {
		format!("`{}` hugs `{}`! ^w^\n{}", author, target, Self::get_rand())
	}
}
