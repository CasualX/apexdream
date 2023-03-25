use std::{cell::Cell, mem};

/// Dynamically constructed strings in the API.
pub struct StringPool {
	strings: Cell<Vec<String>>,
}
impl Default for StringPool {
	fn default() -> Self {
		StringPool::new()
	}
}
impl StringPool {
	pub const fn new() -> StringPool {
		StringPool { strings: Cell::new(Vec::new()) }
	}
	pub fn clear(&mut self) {
		let mut strings = self.strings.replace(Vec::new());
		strings.clear();
		mem::forget(self.strings.replace(strings));
	}
	pub fn store(&self, s: String) -> &str {
		let mut strings = self.strings.replace(Vec::new());
		let s_ptr = s.as_str() as *const str;
		strings.push(s);
		mem::forget(self.strings.replace(strings));
		unsafe { &*s_ptr }
	}
}
