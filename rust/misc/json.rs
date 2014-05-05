extern mod extra;
use extra::json;

fn main() {
	let js = stringify!(
		"hi" : "yo",
		"hey" : "5",
		"config" : {
			"lol" : "ggdsg"
		}

	);

	println!("coding: {}", js);
}
