use std::{fmt::{self, Write}, mem};
use pelite::pe64::*;
use pelite::pattern as pat;
use pelite::{Pod, util::CStr};
use super::ident;

pub fn print(f: &mut super::Output, bin: PeFile) {
	let tables = tables(bin);

	let _ = fmtools::write! { f.human,
		"## RecvTables\n\n"
		for table in &tables {
			"<details>\n"
			"<summary><code>class "{ident(table.name)}
			if let Some(base) = table.base {
				" extends "{base}
			}
			"</code></summary>\n\n"
			"```\n{\n"
			for prop in &table.props {
				"\t"{ident(prop.name)}": "{prop.ty}",\n"
			}
			"}\n```\n\n"
			"</details>\n"
		}
		"\n"
	};
	let _ = fmtools::write! { f.ini,
		for table in &tables {
			"[RecvTable."{ident(table.name)}"]\n"
			for prop in &table.props {
				{ident(prop.name)}"="{prop.offset;#06x}"\n"
			}
			"\n"
			"[RecvTableTypes."{ident(table.name)}"]\n"
			if let Some(base) = table.base {
				"@extends="{base}"\n"
			}
			for prop in &table.props {
				{ident(prop.name)}"="{prop.ty}"\n"
			}
			"\n"
		}
	};
}

#[derive(Copy, Clone, Pod)]
#[repr(C)]
struct RecvTable {
	inst: Ptr,
	props: Ptr<[Ptr<RecvProp>]>, // Goes through heap :(
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
				if name.contains("\"") || name.contains("\n") {
					write!(f, "DataTable(?)")
				}
				else {
					name.fmt(f)
				}
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

	// Because RecvTable.props goes through heap, find their constructors instead...

/*
4C 8D 0D ?? ?? ?? ??            lea     r9, RecvTableName
41 B8 ?? ?? ?? ??               mov     r8d, num props
48 8D 15 ?? ?? ?? ??            lea     rdx, g_RecvProps
48 8D 0D ?? ?? ?? ??            lea     rcx, g_RecvTable
E8 ?? ?? ?? ??                  call    constructor
sometimes an extra instruction here
B8 01 00 00 00                  mov     eax, 1
*/
	let mut matches = bin.scanner().matches_code(pat!("4C8D0D${'} 41B8u4 488D15${'} 488D0D${'} E8${}"));
	while matches.next(&mut save) {
		if let Ok(table) = table(bin, save[4], save[3]) {
			list.push(table);
		}
	}

/*
48 8D 15 ?? ?? ?? ??            lea     rdx, g_RecvProps
41 B8 ?? ?? ?? ??               mov     r8d, num props
48 8D 0D ?? ?? ?? ??            lea     rcx, g_RecvTable
E8 ?? ?? ?? ??                  call    constructor
sometimes an extra instruction here
B8 01 00 00 00                  mov     eax, 1
*/
	let mut matches = bin.scanner().matches_code(pat!("488D15${'} 41B8???? 488D0D${'} E8${}"));
	while matches.next(&mut save) {
		if let Ok(table) = table(bin, save[2], save[1]) {
			list.push(table);
		}
	}

/*
48 8D 0D ?? ?? ?? ??            lea     rcx, RecvTableName
C7 05 ?? ?? ?? ?? ?? ?? ?? ??   mov     g_RecvProps, num props
48 89 0D ?? ?? ?? ??            mov     g_RecvName, rcx
48 8D 0D ?? ?? ?? ??            lea     rcx, g_RecvTable
*/
	let mut matches = bin.scanner().matches_code(pat!("488D0D${'} C705${????'}u20000 48890D???? 488D0D${'}"));
	while matches.next(&mut save) {
		let recv_table = save[2] - 16;
		let recv_props = save[4];
		if let Ok(table) = table(bin, recv_table, recv_props) {
			list.push(table);
		}
	}

	list.sort_unstable_by_key(|table| table.name);
	return list;
}

fn read_prop<'a>(bin: PeFile<'a>, prop: &'a RecvProp) -> pelite::Result<Prop<'a>> {
	let name = bin.deref_c_str(prop.name)?.to_str()?;
	let offset = prop.offset as u32;
	let ty = if let Ok(table) = bin.deref(prop.data_table) {
		let name = bin.deref_c_str(table.name)?.to_str()?;
		Type::DataTable(name)
	}
	else {
		Type::SendPropType(prop.ty)
	};
	let flags = prop.flags;
	let num_elements = prop.num_elements;
	Ok(Prop { name, offset, ty, flags, num_elements })
}

fn table<'a>(bin: PeFile<'a>, recv_table: u32, recv_props: u32) -> pelite::Result<Table<'a>> {
	let offset = recv_table;
	let recv_table = bin.derva::<RecvTable>(recv_table)?;
	let recv_props = bin.derva_slice::<RecvProp>(recv_props, recv_table.num_props as usize)?;
	let name = bin.deref_c_str(recv_table.name)?.to_str()?;
	let mut props = Vec::new();
	let mut first = true;
	let mut base = None;
	for recv_prop in recv_props {
		let propx = match read_prop(bin, recv_prop) {
			Ok(prop) => prop,
			Err(_) => continue,
		};
		// The first prop is the base class
		match propx.ty {
			Type::DataTable(name) if first && propx.offset == 0 => base = Some(name),
			_ => props.push(propx),
		}
		first = false;
	}
	props.sort_by_key(|prop| prop.offset);
	Ok(Table { name, offset, base, props })
}
