fn main() {
	do 10.times {
		do spawn {
			let greeting = "hello!";
			println!("{}",greeting);
		}
	}
}
