fn if_else_assignment() {
	/* you don't need to repeat the variable in rust */
	let z = 99;

	println("if_else_assignment:");

	let result =
	if z == 98 {
		98
	} else if z == 99 {
		99
	} else {
		100
	};
	println!("\tresult={}",result);
}



fn pattern_matching(a: &str) {
	println("pattern_matching:");

	match a {
		"hi" => println("\thi"),
		"derp"|"darpa" => println("\tDARPA!"),
		_ => println!("\twhat {}", a)
	}
}

fn pattern_matching_ints(a: int) {
	/* matches must have an arm covering every branch */
	println("pattern_matching_ints:");
	match a {
		1 => println("\t1"),
		2..5 => println("\t2..5"),
		7|99 => println!("\t7|99 => {}", a),
		/* _ matches any single value */
		_ => println!("\twhat {}", a)
	}
}

fn main() {
	if_else_assignment();

	let _str = ~"hello";
	pattern_matching(_str);
	pattern_matching("hi");
	pattern_matching("darpa");
	pattern_matching_ints(1);
	pattern_matching_ints(3);
	pattern_matching_ints(99);
	pattern_matching_ints(100000000);
}
