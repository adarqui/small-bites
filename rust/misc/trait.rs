trait ExtraMethods {
	fn boop(&self);
	fn boop2(&self);
}

impl ExtraMethods for int {
	fn boop(&self) {
		println("boop");
	}
	fn boop2(&self) {
		println!("boop {}", self.to_str());
	}
}

fn main() {
	let x : int = 1;
	x.boop();
	x.boop2();
}
