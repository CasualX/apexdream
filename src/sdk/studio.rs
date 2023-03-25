use std::mem;
use super::{Ptr, Pod, CUtlVector};

#[derive(Debug, Default)]
#[repr(C)]
pub struct CStudioHdr {
	pub m_pVModel: Ptr,
	pub m_pStudioHdr: Ptr<studiohdr_t>,
	pub m_pStudioHdrCache: CUtlVector<Ptr>,
	pub m_nFrameUnlockCounter: i32,
	pub m_pFrameUnlockCounter: Ptr<i32>,
}

#[derive(Pod, Debug)]
#[repr(C)]
pub struct studiohdr_t {
	pub pad1: [u8; 50],
	pub numhitboxsets: u16,
	pub hitboxsetindex: u16,
	pub pad2: [u8; 62],
	pub numbones: u16,
	pub boneindex: u16,
}
impl Default for studiohdr_t {
	fn default() -> Self {
		dataview::zeroed()
	}
}
impl studiohdr_t {
	pub fn hitboxsetoffset(&self) -> u32 {
		(self.hitboxsetindex as u32 & 0xfffe) << (4 * (self.hitboxsetindex as u32 & 0x1))
	}
	pub fn boneoffset(&self) -> u32 {
		(self.boneindex as u32 & 0xfffe) << (4 * (self.boneindex as u32 & 0x1))
	}
}
const _: [(); 0x32] = [(); dataview::offset_of!(studiohdr_t.numhitboxsets)];
const _: [(); 0x74] = [(); dataview::offset_of!(studiohdr_t.numbones)];

#[derive(Pod, Debug, Default)]
#[repr(C)]
pub struct mstudiobone_t {
	pub one: i32,
	pub unk1: u16,
	pub unk2: u16,
	pub parent: u16,
	pub unk3: u16,
}
const _: [(); 12] = [(); mem::size_of::<mstudiobone_t>()];

#[derive(Pod, Debug, Default)]
#[repr(C)]
pub struct mstudiohitboxset_t {
	pub sznameindex: u16,
	pub numhitboxes: u16,
	pub hitboxindex: u16,
}
impl mstudiohitboxset_t {
	pub fn hitboxoffset(&self) -> u32 {
		(self.hitboxindex as u32 & 0xfffe) << (4 * (self.hitboxindex as u32 & 0x1))
	}
}

#[derive(Pod, Debug, Default)]
#[repr(C)]
pub struct mstudiobbox_t {
	pub bone: u16,
	pub group: u16,
	pub bbmin: [f32; 3],
	pub bbmax: [f32; 3],
	pub szhitboxnameindex: u16,
	pub unused: u16,
}
impl mstudiobbox_t {
	pub fn radius(&self) -> f32 {
		let size = super::sub(self.bbmax, self.bbmin);
		let volume = super::dot(size, size);
		f32::cbrt(volume / (4.0 / 3.0 * std::f32::consts::PI))
	}
}
const _: [(); 0x20] = [(); mem::size_of::<mstudiobbox_t>()];

unsafe impl Pod for CStudioHdr {}
