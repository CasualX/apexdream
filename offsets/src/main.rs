use std::{env, fmt};
use std::path::PathBuf;

mod analysis;

fn parse_arg() -> Option<(PathBuf, bool)> {
	let mut args_os = env::args_os();
	args_os.next()?;
	let path = args_os.next().map(|path| path.into())?;
	let human = args_os.next().map(|arg| {
		let arg = arg.to_string_lossy();
		if arg == "human" { true }
		else if arg == "ini" { false }
		else { panic!("Expected `human` or `ini` argument!") }
	}).unwrap_or(true);

	Some((path, human))
}

pub fn print_error(error: impl fmt::Display) {
	eprintln!("{}", error);
}

fn main() {
	match parse_arg() {
		None => {
			eprintln!("Give the path to a dump apex binary.");
			return;
		},
		Some((path, human)) => {
			let filemap = pelite::FileMap::open(&path).unwrap();
			let mut output = analysis::Output::default();
			analysis::parse(&mut output, filemap.as_ref());
			let s = if human { &output.human } else { &output.ini };
			print!("{}", s);
		},
	}
}
