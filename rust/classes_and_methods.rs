enum Shape {
	Point(int,int),
}

impl Shape {
	fn draw(&self) {
		println("draw:");
		match *self {
			Point(a,b) => { println!("\tdraw! {} {}", a,b); }
		}
	}
}

fn main() {
	let s = Point(1,1);
	s.draw();
}
