use format_xml::template;
use pelite::pe64::*;
use pelite::pattern as pat;
use pelite::{Pod, util::CStr};
use std::{fmt, mem};

pub fn print(bin: PeFile, _dll_name: &str) {
	let tables = tables(bin);

	template::print! {
		"## RecvTables\n\n"
		for table in (&tables) {
			"<details>\n"
			"<summary><code>class "{table.name}
			if let Some(base) = (table.base) {
				" extends "{base}
			}
			"</code></summary>\n\n"
			"```\n{\n"
			for prop in (&table.props) {
				"\t"{prop.name}": "{prop.ty}",\n"
			}
			"}\n```\n\n"
			"### Offsets\n\n"
			"```\n"
			for prop in (&table.props) {
				{table.name}"!"{prop.offset;#06x}" "{prop.name}"\n"
			}
			"```\n"
			"</details>\n"
		}
		"\n"
	}
}

#[derive(Copy, Clone, Pod)]
#[repr(C)]
struct RecvTable {
	inst: Ptr,
	props: Ptr<Ptr<RecvProp>>, // Goes through heap :(
	num_props: i32,
	_unk0: [u32; 256],
	_unk1: [u32; 43],
	decoder: Ptr,
	name: Ptr<CStr>,
	initialized: u8,
	in_main_list: u8,
	_pad: [u8; 6],
} // sizeof: 0x4D8
const _: [(); mem::size_of::<RecvTable>()] = [(); 0x4D8];

#[derive(Copy, Clone, Pod)]
#[repr(C)]
struct RecvProp {
	ty: i32,
	offset: i32,
	_unk1: [u32; 6],
	data_table: Ptr<RecvTable>,
	name: Ptr<CStr>,
	is_inside_array: u8,
	_pad0: [u8; 7],
	array_prop: Ptr<RecvProp>,
	proxy_fn: Ptr,
	_unk2: [u32; 4],
	flags: i32,
	_unk3: u32,
	num_elements: i32,
	_pad1: u32,
} // sizeof: 0x68
const _: [(); mem::size_of::<RecvProp>()] = [(); 0x68];

static SEND_PROP_TYPES: [&str; 11] = [
	"Int", "Float", "Vector", "VectorXY", "String",
	"Array", "Rotation", "BitMask", "Cycle", "Time",
	"DataTable",
];

#[derive(Copy, Clone, Debug)]
enum Type<'a> {
	SendPropType(i32),
	DataTable(&'a str),
}
impl fmt::Display for Type<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Type::SendPropType(ty) => {
				if let Some(name) = SEND_PROP_TYPES.get(*ty as usize) {
					name.fmt(f)
				}
				else {
					write!(f, "SendPropType({})", ty)
				}
			},
			Type::DataTable(name) => {
				name.fmt(f)
			},
		}
	}
}
#[derive(Copy, Clone, Debug)]
struct Prop<'a> {
	name: &'a str,
	offset: u32,
	ty: Type<'a>,
	flags: i32,
	num_elements: i32,
}
#[derive(Clone, Debug)]
struct Table<'a> {
	name: &'a str,
	offset: u32,
	base: Option<&'a str>,
	props: Vec<Prop<'a>>,
}

fn tables<'a>(bin: PeFile<'a>) -> Vec<Table<'a>> {
	let mut save = [0; 8];
	let mut list = Vec::new();
	let mut matches = bin.scanner().matches_code(pat!("4C8D0D${} 41B8???? 488D15${'} 488D0D${'} E8${} B801000000"));
	while matches.next(&mut save) {
		if let Ok(table) = table(bin, (save[2], save[1])) {
			list.push(table);
		}
	}
	let mut matches = bin.scanner().matches_code(pat!("8B? 488D15${'} 41B8???? 488D0D${'} E8${} B801000000"));
	while matches.next(&mut save) {
		if let Ok(table) = table(bin, (save[2], save[1])) {
			list.push(table);
		}
	}
	list.sort_unstable_by_key(|table| table.name);
	return list;
}

fn table<'a>(bin: PeFile<'a>, (recv_table, recv_props): (u32, u32)) -> pelite::Result<Table<'a>> {
	let offset = recv_table;
	let recv_table = bin.derva::<RecvTable>(recv_table)?;
	let recv_props = bin.derva_slice::<RecvProp>(recv_props, recv_table.num_props as usize)?;
	let name = bin.deref_c_str(recv_table.name)?.to_str().unwrap();
	let mut props = Vec::new();
	let mut first = true;
	let mut base = None;
	for prop in recv_props {
		let name = bin.deref_c_str(prop.name).unwrap().to_str().unwrap();
		let offset = prop.offset as u32;
		let ty = if let Ok(table) = bin.deref(prop.data_table) {
			let name = bin.deref_c_str(table.name).unwrap().to_str().unwrap();
			Type::DataTable(name)
		}
		else {
			Type::SendPropType(prop.ty)
		};
		let flags = prop.flags;
		let num_elements = prop.num_elements;
		// The first prop is the base class
		match ty {
			Type::DataTable(name) if first && offset == 0 => base = Some(name),
			_ => props.push(Prop { name, offset, ty, flags, num_elements }),
		}
		first = false;
	}
	props.sort_by_key(|prop| prop.offset);
	Ok(Table { name, offset, base, props })
}
