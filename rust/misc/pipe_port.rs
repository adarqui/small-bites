use std::libc;

fn main() {
	let (port,chan):(Port<int>,Chan<int>) = stream();
	do spawn {
		unsafe {
			chan.send(10);
		}
	}
	println!("{}",port.recv().to_str());
}
