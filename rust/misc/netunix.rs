use std::os;
use std::rt::io::net::unix::UnixStream;
use std::rt::io::net::unix::UnixListener;
use std::rt::io::net::unix::UnixAcceptor;
use std::rt::io::Listener;
use std::rt::io::Acceptor;
use std::rt::io::Writer;
use std::rt::io::Reader;
use std::rt::io::net::unix;
use std::path::Path;
use std::rt::io;
use std::rt::io::timer;

fn server(path: ~str) {
	let mut chan = Path::new(path);
	/*
	do io::io_error::cond.trap(|_| {
		println("error");
		error!("bye");
	}).inside {
		*/
		let mut stream = UnixListener::bind(&chan).listen();

		loop {
			let mut cli = stream.accept();
			do spawn {
				cli.write([1,2,3,4]);
				println("spawn");
			}
			timer::sleep(1000);
		}
		/*
	}
	*/
}

fn client(path: ~str) {
	let chan = Path::new(path);
	let mut stream = UnixStream::connect(&chan);
	loop {
		stream.write([1,2,3,4]);
		timer::sleep(1000);
	}
}

fn main() {
	let argv = os::args();
	let mode = argv[1].clone();
	let path = argv[2].clone();

	match mode {
		~"server" => {
			println("Server!");
			server(path);
		}
		~"client" => {
			println("Client!");
			client(path);
		}
		_ => {
			println("Unknown!");
		}
	}
}
