extern mod extra;
use extra::time;

fn timed(label: &str, f: &fn()) {
	let start = time::precise_time_s();
	f();
	let end = time::precise_time_s();
	println!("{} = {}", label, end-start);
}

fn main() {
	do timed("loop") {
		for i in range(1,10) {
			println!("hi {}",i);
		}
	}
}
