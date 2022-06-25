use std::fmt::Write;
use pelite::pattern as pat;
use pelite::pe64::*;

pub fn print(f: &mut super::Output, bin: PeFile<'_>) {
	let sts = string_tables(bin);

	let _ = fmtools::write! { f.human,
		"## NetworkedStringTables\n\n```\n"
		for st in &sts {
			{st.name}"\n"
		}
		"```\n\n"
	};
	let _ = fmtools::write! { f.ini,
		"[NetworkedStringTables]\n"
		for st in &sts {
			{st.name}"="{st.offset:#010x}"\n"
		}
		"\n"
	};
}

struct StringTable<'a> {
	name: &'a str,
	offset: u32,
}

fn string_tables(bin: PeFile<'_>) -> Vec<StringTable<'_>> {
	let mut save = [0; 4];
	let mut sts = Vec::new();

	let mut matches = bin.scanner().matches_code(pat!("488D15${'} 498BCB E8${4C8BC9} [1-50] 488905$'"));
	while matches.next(&mut save) {
		let name = bin.derva_c_str(save[1]).unwrap().to_str().unwrap();
		// if let Ok(name) =  {
			let offset = save[2];
			sts.push(StringTable { name, offset });
		// }
	}

	sts.sort_by_key(|st| st.name);
	return sts;
}
