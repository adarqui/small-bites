use std::os;
fn main() {
	let argv = os::args();
	println!("argv0 = {}", argv[0].to_str());
	println!("{} {}", argv[1], argv.len());
	for a in argv.iter() {
		println!("{}",a.to_str());
	}
}
