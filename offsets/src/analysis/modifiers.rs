use std::fmt::Write;
use pelite::pattern as pat;
use pelite::pe64::*;

pub fn print(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0u32; 4];
	if !bin.scanner().finds_code(pat!("488D0D${\"Attempted to perform GetMods\"} [40-80] 399Eu4 [2-10] 8D?u4 [20-50] 488B15${*'}"), &mut save) {
		crate::print_error("unable to find modifiers");
		return;
	}

	// The weapon entity at m_weaponNameIndex + 0x14:
	//  A pointer to a structure containing available modifiers for this weapon
	//  At modifiers + mods_count is an i32 which is the number of mods for this weapon
	//  At modifiers + mods_list is an array of u16 indices with stride 10 bytes
	//  Read the index and offset into the array of cstrings at r5apex!mods_names to know what mod it is

	let mods_count = save[1];
	let mods_list = save[2];
	let mods_names = save[3];

	let mut names = Vec::new();
	for i in 0.. {
		let ptr = match bin.derva(mods_names + i * 8) {
			Ok(&ptr) => ptr,
			Err(_) => break,
		};

		let name = match bin.deref_c_str(ptr) {
			Ok(name) => match name.to_str() {
				Ok(name) => name,
				Err(_) => break,
			},
			Err(_) => break,
		};

		// Last entry appears to be an empty string?
		if name.is_empty() {
			break;
		}

		names.push(name);
	}

	let _ = fmtools::write! { f.ini,
		"[ModifierOffsets]\n"
		"mods_names="{mods_names:#x}"\n"
		"mods_list="{mods_list:#x}"\n"
		"mods_count="{mods_count:#x}"\n"
		"\n"
		"[ModifierNames]\n"
		for (index, &name) in names.iter().enumerate() {
			{name}"="{index}"\n"
		}
		"\n"
	};

	names.sort_unstable();
	names.dedup();
	let _ = fmtools::write! { f.human,
		"## ModifierNames\n\n```\n"
		for &name in &names {
			{name}"\n"
		}
		"```\n\n"
	};
}
