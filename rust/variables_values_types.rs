fn local_variables() {
	/* a cannot change */
	let a = 1;
	/* but b can, because of mut */
	let mut b = 2;
	b = 3;
	println("local_variables:");
	println!("\ta={} {}",a,b);
}


fn explicit_type() {
	/* rust will infer type, but, to specify, follow with : <type> = .. */
	let a: int = 100;
	static DEFINITION: int = 2;
	println("explicit_type:");
	println!("\ta {}, DEFINITION {}", a, DEFINITION);
}



fn silencing_unused_variables() {
	/* to stop the compiler from whining, prefix the variable with an underscore _ */
	let _a = 100000;
	println("silencing_unused_variables: prefix with _");
}



fn types() {
	println("type:");

	let _int: int = -5;
	let _int2 = -5i;
	let _uint: uint = 10;
	let _uint2 = 100u;
	let _f64: f64 = 5.643;
	let _bool: bool = true;
	let _char: char = 'c';
	let _string = r##"hi"##;

	println!("_int={}, _int2={}, _uint={}, _uint2={}, _f64={}, _bool={}, _char={}, _string={}", _int,_int2,_uint,_uint2,_f64,_bool,_char,_string);
}


fn create_a_new_type() {
	/* declare it with a capital letter */
	type Int = int;
	let mut z: Int = 99;
	println("create_a_new_type:");
	println!("\tz={}",z);
}


fn type_conversion() {
	let _f64: f64 = 3.1456;
	let _uint: uint = _f64 as uint;
	println("type_conversion");
	println!("\t_f64={}, _uint={}", _f64,_uint);
}


struct Point {
	x : f64,
	y : f64,
}

fn structures() {

	println("structures:");
	/* can change p's fields */
	let mut p = Point { x : 1.0, y : 2.1 };

	/* can't change p_nonmut's fields */
	let p_nonmut = Point { x: 5.5, y: 6.6 };

	println!("\tp.x={},p.y={}",p.x,p.y);
}



enum C_enum {
	BOOP,
	DOOP,
	ZOOP
}


enum C_enum_2 {
	OPT1 = 0xff000000,
	OPT2 = 0x00ff0000,
	OPT3 = 0x0000ff00,
	OPT4 = 0xffffffff,
}

enum Rust_enum {
	Circle(int, f64),
	Square(int, int)
}


/*
fn match_rust_enum(a: Rust_enum) -> int {
	match a {
		Circle(a,_) => a*3,
		Square(_,b) => b*5,
	}
	return 1;
}
*/


fn tuple() {
	println("tuple:");
	let tup: (int,int,int) = (1,2,3);
	match tup {
		(a, b, c) => println!("\t{} {} {}", a,b,c)
	}
}

fn tuple_struct() {
	struct Tuple(int,int,int);
	let tup: Tuple = Tuple(9,9,9);
	match tup {
		Tuple(a,b,c) => println!("\t{} {} {}", a,b,c)
	}
}

fn main() {
	local_variables();
	explicit_type();
	silencing_unused_variables();
	types();
	create_a_new_type();
	type_conversion();
	structures();
//	match_rust_enum((5,6));
	tuple();
	tuple_struct();
}
