
use std::env;
use std::path::PathBuf;

macro_rules! tprint {
	($($tt:tt)*) => {
		print!("{}", format_xml::template!{$($tt)*});
	};
}

fn parse_arg() -> Option<PathBuf> {
	let mut args_os = env::args_os();
	args_os.next();
	args_os.next().map(|path| path.into())
}

fn main() {
	match parse_arg() {
		None => {
			eprintln!("Give the path to a dump apex binary.");
			return;
		},
		Some(path) => {
			let filemap = pelite::FileMap::open(&path).unwrap();
			parse(filemap.as_ref());
		},
	}
}

mod interfaces;
mod classes;
mod recvtables;
mod datamaps;
mod misc;
mod kbuttons;
mod convars;
mod concommands;
mod globals;

fn parse(image: &[u8]) {
	use pelite::pe64::*;
	let bin = PeFile::from_bytes(image).unwrap();
	let dll_name = bin.exports().unwrap().dll_name().unwrap().to_str().unwrap();
	interfaces::print(bin, dll_name);
	misc::print(bin, dll_name);
	kbuttons::print(bin, dll_name);
	classes::print(bin, dll_name);
	recvtables::print(bin, dll_name);
	datamaps::print(bin, dll_name);
	convars::print(bin, dll_name);
	concommands::print(bin, dll_name);
	globals::print(bin, dll_name);
}
