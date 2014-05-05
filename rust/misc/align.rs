use std::os;

fn align_to(size: uint, align: uint) -> uint {
	    assert!(align != 0);
		    (size + align - 1) & !(align - 1)
}

fn print_uint(x:uint) {
	println!("{}",x);
}

fn main() {
	let argv = os::args();
	let size = from_str::<uint>(argv[1]).unwrap();
//	println!("{}",size);

	let align = from_str::<uint>(argv[2]).unwrap();
//	println!("{}", align);

	let aligned = align_to(size,align);
	println!("{} by {} = {}", size, align, aligned);

//	print_uint(*argv[1]);
}
