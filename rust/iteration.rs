fn while_loop(y: int) -> int {
	println("while_loop:");
	let mut x: int = 0;
	while x < y {
		println!("\tx = {}", x);
		x += 1;

		if x == y { break; }
		if x != y { continue; }
	}
	return 0;
}


fn for_loop(y: int, z: int) -> int {
	println("for_loop:");
	let mut x: int = y;
	/* while true */
	loop {
		x += x;
		if x % z == 0 { break; }
		println!("x = {}", x.to_str());
	}
	return 0;
}


fn each(v: &[int], op: &fn(v: int)) {
	let mut n = 0;
	while n < v.len() {
		op(v[n]);
		n+=1;
	}
}

fn closure_iteration(x: &[int]) -> (int,int) {
	println("closure_iteration:");
	let mut n: int = 0;
	let mut tot: int = 0;
	do each(x) |i| {
		//n = n + i;
		n=n+1;
		tot = tot + i;
		println!("\ti={}",i);
	}
	println!("\tn={}, tot={}",n,tot);
	return (n,tot);
}


fn main() {
	while_loop(5);
	for_loop(100,3);
	let mut a: int = 0;
	let mut b: int = 0;
	closure_iteration([1,2,3,4,5,6,7]);
}
