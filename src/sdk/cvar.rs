use super::{Pod, Ptr};

#[derive(Pod, Debug)]
#[repr(C)]
pub struct RawConVar {
	// ConCommandBase
	pub vtable: u64,
	pub pNext: Ptr<RawConVar>,
	pub bRegistered: u8,
	_pad0: [u8; 7],
	pub pszName: u64,
	pub pszHelpString: u64,
	pub pszDataType: u64,
	unk_u64: u64,
	pub fFlags: u32,
	_pad1: u32,
	// ConVar
	pub IConVar_vtable: u64,
	pub pParent: Ptr<RawConVar>,
	pub pszDefaultValue: u64,
	pub pszString: u64, // Allocated
	pub StringLength: u64,
	pub fValue: f32,
	pub nValue: i32,
	pub bHasMin: u8,
	pub bHasMax: u8,
	_pad2: [u8; 2],
	pub fMinVal: f32,
	pub fMaxVal: f32,
	_pad3: u32,
	// Callback stuff...
	// callback_stuff: [u64; 4],
}
