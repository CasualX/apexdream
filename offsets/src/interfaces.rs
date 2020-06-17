
use pelite;
use pelite::pe64::*;
use pelite::pattern as pat;

pub fn print(bin: PeFile, dll_name: &str) {
	let ifaces = interfaces(bin);

	tprint! {
		"## Interfaces\n\n"
		"```\n"
		for iface in (&ifaces) {
			{dll_name}"!"{iface.address;#010x}" "{iface.name}"\n"
		}
		"```\n\n"
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Interface<'a> {
	pub name: &'a str,
	pub address: u32,
}

pub fn interfaces(bin: PeFile<'_>) -> Vec<Interface<'_>> {
	let mut save = [0; 8];
	let mut list = Vec::new();

	let mut matches = bin.scanner().matches_code(pat!("488B05${} 488905${} 488D05${*{488D05$'}*'} 488905${} C3"));
	while matches.next(&mut save) {
		let address = save[1];
		let name = bin.derva_c_str(save[2]).unwrap().to_str().unwrap();
		list.push(Interface { name, address });
	}

	if list.is_empty() {
		eprintln!("unable to find any interfaces!");
	}

	list.sort_by_key(|item| item.name);
	list
}
