use std::fmt::Write;
use format_xml::template;
use pelite;
use pelite::pe64::*;
use pelite::{util::CStr, Pod};
use pelite::pattern as pat;

pub fn print(f: &mut super::Output, bin: PeFile) {
	let cvars = convars(bin);

	let _ = template::write! { f.human,
		"## ConVars\n\n"
		for cvar in (&cvars) {
			"<details>\n"
			"<summary><code>"{cvar.name}"</code></summary>\n\n"
			if let Some(desc) = (cvar.desc) {
				{desc}"\n\n"
			}
			"default: `"{cvar.default;?}"`  \n"
			"flags: `"{cvar.flags;#x}"`  \n"
			if let Some(min_value) = (cvar.min_value) {
				"min value: `"{min_value}"`  \n"
			}
			if let Some(max_value) = (cvar.max_value) {
				"max value: `"{max_value}"`  \n"
			}
			"</details>\n"
		}
		"\n"
	};
	let _ = template::write! { f.ini,
		"[ConVars]\n"
		for cvar in (&cvars) {
			{cvar.name}"="{cvar.address;#010x}"\n"
		}
		"\n"
	};
}

// Find information in the 'setinfo' command
// References "Custom user info value"
// sizeof(ConVar) == 160
#[allow(non_snake_case)]
#[derive(Pod, Debug)]
#[repr(C)]
pub struct RawConVar {
	// ConCommandBase
	pub vtable: u64,
	pub pNext: Ptr<RawConVar>,
	pub bRegistered: u8,
	_pad0: [u8; 7],
	pub pszName: Ptr<CStr>,
	pub pszHelpString: Ptr<CStr>,
	pub pszDataType: Ptr<CStr>,
	unk_u64: u64,
	pub fFlags: u32,
	_pad1: u32,
	// ConVar
	pub IConVar_vtable: u64,
	pub pParent: Ptr<RawConVar>,
	pub pszDefaultValue: Ptr<CStr>,
	pub pszString: u64, // Allocated
	pub StringLength: u64, // Length of allocated string
	pub fValue: f32,
	pub nValue: i32,
	pub bHasMin: u8,
	_pad2: [u8; 3],
	pub fMinVal: f32,
	pub bHasMax: u8,
	_pad3: [u8; 3],
	pub fMaxVal: f32,
	// Callback stuff...
	// callback_stuff: [u64; 4],
}

pub struct ConVar<'a> {
	pub address: u32,
	pub name: &'a str,
	pub desc: Option<&'a str>,
	pub data_type: &'a str,
	pub default: &'a str,
	pub flags: u32,
	pub min_value: Option<f32>,
	pub max_value: Option<f32>,
}

pub fn convars(bin: PeFile<'_>) -> Vec<ConVar<'_>> {
	// Find the main ConVar vtable
	// Alternative slow pattern: @3 *{'*{}} *{} [8] *"thirdperson_override" (just find a specific ConVar, in .data)
	let mut save = [0; 4];
	if !bin.scanner().finds_code(pat!("C6411000 33C9 ?8D?${'}"), &mut save) {
		crate::print_error(&"ERR: unable to find ConVar vftable");
		return Vec::new();
	}
	// Get the virtual address of the ConVar vtable
	let vftable = bin.optional_header().ImageBase + save[1] as u64;
	// Find the data section
	let data_section = bin.section_headers().iter().find(|section| &section.Name == b".data\0\0\0").unwrap();
	// Scan the data section for pointers to the vtable
	let data_data = bin.derva_slice::<u64>(data_section.VirtualAddress, data_section.SizeOfRawData as usize / 8).unwrap();
	let mut convars = Vec::new();
	for i in data_data.iter().enumerate().filter_map(|(index, &ptr)| if ptr == vftable { Some(index) } else { None }) {
		let address = data_section.VirtualAddress + (i * 8) as u32;
		let raw = bin.derva::<RawConVar>(address).unwrap();
		let name = bin.deref_c_str(raw.pszName).unwrap_or(CStr::empty()).to_str().unwrap();
		let desc = bin.deref_c_str(raw.pszHelpString).ok().map(|desc| desc.to_str().unwrap());
		let data_type = bin.deref_c_str(raw.pszDataType).unwrap_or(CStr::empty()).to_str().unwrap();
		let default = bin.deref_c_str(raw.pszDefaultValue).unwrap_or(CStr::empty()).to_str().unwrap();
		let flags = raw.fFlags;
		let min_value = if raw.bHasMin != 0 { Some(raw.fMinVal) } else { None };
		let max_value = if raw.bHasMax != 0 { Some(raw.fMaxVal) } else { None };
		convars.push(ConVar { address, name, desc, data_type, default, flags, min_value, max_value });
	}

	// Sort to make a nice diff
	convars.sort_by_key(|convar| convar.name);
	convars
}
