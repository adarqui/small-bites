struct Object {
	age : int
}

impl Object {
	fn new(age:int) -> Object {
		Object { age : age }
	}
	fn print(&self) {
		println!("age = {}", self.age);
	}
}

fn main() {
	let x = Object { age : 31 };
	println!("{}",x.age);

	let y = Object::new(10);
	println!("{}",y.age);

	y.print();
}
