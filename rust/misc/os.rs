use std::os;
fn main() {
	let x = os::getcwd();
	println!("cwd={}",x.display());
}
