extern mod extra;
use extra::uuid::Uuid;
use extra::base64;

fn main() {
	let s = Uuid::new_v4().to_str();
	println!("{}", s);
	let b = s.as_bytes().to_base64(base64::STANDARD);
}
