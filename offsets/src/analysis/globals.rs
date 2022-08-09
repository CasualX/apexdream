use std::fmt::Write;
use pelite;
use pelite::pe64::*;
use super::ident;

pub fn print(f: &mut super::Output, bin: PeFile) {
	let globals = globals(bin);

	let _ = fmtools::write! { f.ini,
		"[Globals]\n"
		for g in &globals {
			{ident(g.ty_name)}"="{g.address:#010x}"\n"
		}
		"\n"
	};
}

pub struct Global<'a> {
	pub address: u32,
	pub ty_name: &'a str,
}

// Very common instances that aren't very interesting
const BLACKLIST: [&'static str; 12] = [
	".?AVConCommand@@",
	".?AVConVar@@",
	".?AVtype_info@@",
	".?AVCMaterialGlue@@",
	".?AVServerDataBlockSender@@",
	".?AVCClient@@",
	".?AVC_OP_WorldTraceConstraint@@",
	".?AVC_OP_TurbulenceForce@@",
	".?AVCEntityListAlongRay@@",
	".?AVCTraceFilterHitAll@@",
	".?AVSVC_PersistenceUpdateVar@@",
	".?AVCSurface@@",
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
