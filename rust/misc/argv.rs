use std::os;

fn main() {
	let argv = os::args();

	for a in argv.iter() {
		println!("{}",a.to_str());
	}
}
