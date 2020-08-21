use format_xml::template;
use pelite::pattern as pat;
use pelite::pe64::*;

pub fn print(bin: PeFile<'_>, dll_name: &str) {
	let sts = string_tables(bin);
	template::print! {
		"## NetworkedStringTables\n\n```\n"
		for st in (&sts) {
			{dll_name}"!"{st.offset;#010x}" "{st.name}"\n"
		}
		"```\n\n"
	}
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
