extern mod extra;
use extra::glob;
use std::os;

fn main() {
	let argv = os::args();

	let x = glob::glob(argv[1]);

	for path in glob::glob(argv[1]) {
		let mut y = path.dirname().to_str();
		println!("display? {}", path.display());
		println!("exists? {}", path.exists());
		println!("dirname_display? {}", path.filename_display());

		//let mut p = Path::new(path.filename());
		//p.set_filename(argv[2]);
	}
}
