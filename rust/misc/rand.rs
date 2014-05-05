use std::rand;
use std::rand::Rng;

fn main() {
	let mut rng = rand::rng();
	let _int : int = rng.gen();
	let _f64 : f64 = rng.gen();
	println!("hi {} {}", _int,_f64);
}
