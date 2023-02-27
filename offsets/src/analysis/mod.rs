use std::fmt;

mod interfaces;
mod classes;
mod recvtables;
mod datamaps;
mod misc;
mod kbuttons;
mod convars;
mod concommands;
mod globals;
mod string_tables;
mod modifiers;
mod weapondata;

#[derive(Default)]
pub struct Output {
	pub ini: String,
	pub human: String,
}

// Nicely format supposedly valid identifier-like strings
fn ident(s: &str) -> impl '_ + fmt::Display {
	fmtools::fmt! { move
		if s.is_empty() { "{empty}" }
		else if s.contains("\"") || s.contains(" ") || s.contains("\n") { {s:?} }
		else { {s} }
	}
}

pub fn parse(f: &mut Output, image: &[u8]) {
	use pelite::pe64::*;
	let bin = PeFile::from_bytes(image).unwrap();
	interfaces::print(f, bin);
	misc::print(f, bin);
	string_tables::print(f, bin);
	modifiers::print(f, bin);
	kbuttons::print(f, bin);
	classes::print(f, bin);
	recvtables::print(f, bin);
	datamaps::print(f, bin);
	weapondata::print(f, bin);
	convars::print(f, bin);
	concommands::print(f, bin);
	globals::print(f, bin);
}
