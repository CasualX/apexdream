use super::Pod;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[repr(C)]
pub struct kbutton_t {
	pub down: [i32; 2],
	pub state: u32,
}

unsafe impl Pod for kbutton_t {}
