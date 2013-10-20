/*
 * these examples are directly from rust docs
 */

fn simple_destructor() {
	{
		/* as soon as we leave this block, y's memory on heap is free'd */
		let y = ~10;
	}
}


fn boxes() {
	let x = 5;
	let mut y = 5;
	y+=2;

	let a = ~5;
	let mut b = ~5;
	*b+=2;
}


fn move_semantics() {
	let x = ~5;
	let y = x.clone();
	let z = x;


	let r = ~13;
	let mut s = r;
	*s += 1;
	let t = s;
}





struct Point {
	x: f64,
	y: f64,
}

fn compute_distance(p1: &Point, p2: &Point) -> (f64,f64) {
	let x_d: f64 = p1.x - p2.x;
	let y_d: f64 = p1.y - p2.y;
	return (x_d, y_d);
}

fn borrowed_pointers() {
	let on_the_stack : Point = Point { x: 1.2, y: 2.2 };
	let managed_box : @Point = @Point { x : 1.3, y: 2.3 };
	let owned_box : ~Point = ~Point { x: 1.4, y: 2.4 };

	let mut a: f64 = 0.0;
	let mut b: f64 = 0.0;

/*
	(a,b) = compute_distance(&on_the_stack, managed_box);
	println!("\ta={}, b={}", a, b);
	(a,b) = compute_distance(managed_box, owned_box);
	println!("\ta={}, b={}", a, b);
	*/
}


fn dereferencing_pointers() {

	println("dereferencing_pointers:");

	let managed = ~10;
	let owned = ~20;
	let borrowed = &30;

	let sum = *managed + *owned + *borrowed;

	println!("\tsum={}", sum);
}


fn main() {
	simple_destructor();
	boxes();
	move_semantics();
	borrowed_pointers();
	dereferencing_pointers();
}
