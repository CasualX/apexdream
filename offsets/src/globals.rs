use pelite;
use pelite::pe64::*;

pub fn print(bin: PeFile, dll_name: &str) {
	let globals = globals(bin);

	tprint! {
		"## Globals\n\n"
		"List of global variables with an associated vtable and their type name.\n\n"
		"```\n"
		for g in (&globals) {
			{dll_name}"!"{g.address;#010x}" "{g.ty_name}"\n"
		}
		"```\n\n"
	}
}

pub struct Global<'a> {
	pub address: u32,
	pub ty_name: &'a str,
}

// Very common instances that aren't very interesting
const BLACKLIST: [&'static str; 6] = [
	".?AVConCommand@@",
	".?AVConVar@@",
	".?AVtype_info@@",
	".?AVCMaterialGlue@@",
	".?AVServerDataBlockSender@@",
	".?AVCClient@@",
];

pub fn globals(bin: PeFile<'_>) -> Vec<Global<'_>> {
	let image = bin.image();
	let mut globals = Vec::new();
	for i in 0..image.len() / 8 {
		if let Ok(global) = global(bin, i * 8) {
			if !BLACKLIST.iter().any(|&exclude| global.ty_name == exclude) {
				globals.push(global)
			}
		}
	}

	globals.sort_by_key(|g| g.ty_name);
	globals
}

fn global(bin: PeFile<'_>, offset: usize) -> pelite::Result<Global<'_>> {
	let address = bin.file_offset_to_rva(offset)?;
	let vtable_va = *bin.derva::<u64>(address)?;
	let _vtable_rva = bin.va_to_rva(vtable_va)?;
	let col_ptr = *bin.deref::<Ptr<msvc::RTTICompleteObjectLocator>>((vtable_va - 8).into())?;
	let col = bin.deref(col_ptr)?;
	let type_info = bin.derva::<msvc::TypeDescriptor>(col.type_descriptor)?;
	let _ = bin.va_to_rva(type_info.vftable.into())?;
	if type_info.spare != Ptr::null() {
		return Err(pelite::Error::Null);
	}
	let ty_name = bin.derva_c_str(col.type_descriptor + 16)?.to_str()?;
	Ok(Global { address, ty_name })
}
