fn hi() {
	println("hello");
}

fn arguments(a: int, b: int) -> bool {
	println("arguments:");
	if a == b {
		println!("\tequal: {} = {}", a,b);
		return true;
	} else {
		println!("\t!equal: {} != {}", a,b);
		return false;
	}
}

fn arguments2(a: int, b: int) -> bool {
	println("arguments2:");
	let ret =
	if a == b {
		true
	} else {
		false
	};
	println!("\treturn value is: {}", ret);
	return ret;
}


fn no_need_to_return(value: int) -> bool {
	value == 100
}


fn no_need_to_return_but_dont_use_semicolon(value: int) -> () {
	/* with the semicolon, this will return () */
	value == 100;
}

fn multiple_return_values(a: int, b: int) -> (int,int) {
	println!("multiple_return_values: swapping a & b");
	return (b, a);
}


fn argument_unpacking((value,_):(int,int)) -> int {
	value
}



//let closure = |arg| println!("\targ={}", arg);

fn closures_call(b: &fn(int)) { b(10); }

fn closures() {

	let closure = |arg| println!("\targ={}", arg);
	let square = |arg: int| { println!("\targ={}", arg*arg); };

	println("closures:");
	closures_call(closure);
	closures_call(square);

}


fn closures_stack() {
	println("closures_stack:");
	let mut max = 0;
	[1,2,3].map(|x| if *x > max { max = *x });
	println!("\tmax={}",max);
}


fn main() {
	hi();
	let mut ret = arguments(1,2);
	ret = arguments2(2,2);
	println!("no_need_to_return:\n\treturn = {}", no_need_to_return(5));
	let (x,y) = multiple_return_values(1,2);
	println!("x,y = ({},{})", x,y);
	no_need_to_return_but_dont_use_semicolon(5);
	println!("argument_unpacking\n\t{}",argument_unpacking((5,0)));
	closures();
	closures_stack();
}
