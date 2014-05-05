use std::rt::env;

fn main() {
	let _min = env::min_stack();
	println!("min_stack = {}", _min);
}
