use std::fmt::Write;
use std::mem;
use pelite;
use pelite::pe64::*;
use pelite::util::CStr;
use pelite::pattern as pat;

#[derive(dataview::Pod)]
#[repr(C)]
struct RawWeaponDataField {
	name: Ptr<CStr>,
	default: u64,
	description: u64,
	ty: u8,
	flags: u8,
	index: u16,
	offset: u16,
	pad: [u8; 2], // Seems to always be zero
}
const _: [(); 32] = [(); mem::size_of::<RawWeaponDataField>()];

#[derive(Debug)]
enum Value<'a> {
	Null,
	Bool(bool),
	Byte(u8),
	Short(i16),
	Int(i32),
	Float(f32),
	String(&'a str),
	Asset,
	Vector,
	WeaponString,
	Enum(&'a str),
	Special,
}

#[allow(dead_code)]
#[derive(Debug)]
struct WeaponDataField<'a> {
	name: &'a str,
	default: Value<'a>,
	flags: u8,
	index: u16,
	offset: u16,
}

pub fn print(f: &mut super::Output, bin: PeFile) {
	let mut save = [0; 4];
	let mut items = Vec::new();
	// if !bin.scanner().finds_code(pat!("488D?${'} [1-20] 488D?$\"Weapon mod has unknown \"22\"%s\"22\" setting\""), &mut save) {
	// 	return;
	// }
	if !bin.scanner().finds_code(pat!("3Du20000 [1-10] 488D05${'} [30-50] 488D0D${\"Weapon setting '%s' is type %s, not %s.\"} 4F8B84C2${'}"), &mut save) {
		crate::print_error("ERR: unable to find WeaponSettings pattern");
		return;
	}
	let count = (save[1] + 2) as usize;
	let addr = save[2];
	let types = save[3];
	for i in 0u32..count as u32 {
		let _ = read_entry(bin, addr + i * mem::size_of::<RawWeaponDataField>() as u32, &mut items);
	}

	let offset;
	if !bin.scanner().matches_code(pat!("488D0D${\"Error setting mods on weapon \'%s\'. See console log for details.\"} [50-500] 4C8D8Bu20000")).next(&mut save) {
		crate::print_error("ERR: unable to find WeaponSettings base offset");
		offset = 0;
	}
	else {
		offset = save[1];
	}

	items.sort_by_key(|item| item.name);
	let _ = fmtools::write!(f.human,
		"## WeaponSettings\n\n"
		"```\n"
		for item in &items {
			{super::ident(item.name)}": "{item.default:?}"\n"
		}
		"```\n\n"
	);

	items.sort_by_key(|item| item.offset);
	let _ = fmtools::write!(f.ini,
		"[WeaponSettingsMeta]\n"
		"count="{count}"\n"
		"list="{addr:#x}"\n"
		"types="{types:#x}"\n"
		"base="{offset:#06x}"\n"
		"\n"
		"[WeaponSettings]\n"
		for item in &items {
			{super::ident(item.name)}"="{item.offset:#06x}"\n"
		}
		"\n"
	);
}

fn read_entry<'a>(bin: PeFile<'a>, addr: u32, items: &mut Vec<WeaponDataField<'a>>) -> Result<(), pelite::Error> {
	let field = bin.derva::<RawWeaponDataField>(addr)?;

	let name_rva = bin.va_to_rva(field.name.into_raw())?;
	let rdata_sect = bin.section_headers().by_name(".rdata").unwrap();
	if !(name_rva >= rdata_sect.VirtualAddress && name_rva < rdata_sect.VirtualAddress + rdata_sect.VirtualSize) {
		return Err(pelite::Error::Invalid);
	}

	let name = bin.deref_c_str(field.name)?.to_str()?;

	let default = match field.ty {
		0 => Value::Null,
		1 => Value::Bool(field.default & 0xff != 0),
		2 => Value::Byte((field.default & 0xff) as u8),
		3 => Value::Short((field.default & 0xffff) as i16),
		4 => Value::Int((field.default & 0xffffffff) as i32),
		5 => Value::Float(f32::from_bits((field.default & 0xffffffff) as u32)),
		6 => Value::String(if field.default == 0 { "" } else { bin.deref_c_str(field.default.into())?.to_str()? }),
		7 => Value::Asset,
		8 => Value::Vector,
		9 => Value::WeaponString,
		10 => Value::Enum(if field.default == 0 { "" } else { bin.deref_c_str(field.default.into())?.to_str()? }),
		11 => Value::Special,
		_ => return Err(pelite::Error::Insanity),
	};

	items.push(WeaponDataField { name, default, flags: field.flags, index: field.index, offset: field.offset });
	Ok(())
}
