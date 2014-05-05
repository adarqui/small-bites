extern mod extra;
use extra::uuid::Uuid;

fn main() {
	println!("{} {} {}", Uuid::new_v4().to_str(), Uuid::new_v4().to_hyphenated_str(), Uuid::new_v4().to_urn_str());
}
