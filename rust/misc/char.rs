use std::char;

fn main() {
	let c : char = 'a';

	if char::is_lowercase(c) {
		println("lower");
	}

	let res =
	if char::is_lowercase(c) {
		1
	}
	else if char::is_uppercase(c) {
		2
	}
	else  {
		3
	};

	println!("res = {}", res);
}
