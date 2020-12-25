#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

use std::fmt;

mod analysis;

#[allow(dead_code)]
extern {
	fn setINI(ptr: *const u8, len: usize);
	fn setError(ptr: *const u8, len: usize);
	fn setHuman(ptr: *const u8, len: usize);
}

pub fn print_ini(string: &str) {
	unsafe { setINI(string.as_ptr(), string.len()); }
}
pub fn print_human(string: &str) {
	unsafe { setHuman(string.as_ptr(), string.len()); }
}
pub fn print_error<T: fmt::Display>(error: &T) {
	let msg = error.to_string();
	unsafe { setError(msg.as_ptr(), msg.len()); }
}

#[no_mangle]
pub unsafe fn allocate(len: usize) -> *mut u8 {
	let boxed = vec![0u8; len].into_boxed_slice();
	let raw = Box::into_raw(boxed);
	(*raw).as_mut_ptr()
}
#[no_mangle]
pub unsafe fn free(data: *mut [u8]) {
	let boxed = Box::from_raw(data);
	drop(boxed);
}
#[no_mangle]
pub unsafe fn analyze(data: *const [u8]) {
	let data = &*data;
	let mut output = analysis::Output::default();
	analysis::parse(&mut output, data);
	print_ini(&output.ini);
	print_human(&output.human);
}

