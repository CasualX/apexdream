/*!
Shared codebase.
!*/

pub fn unsafe_as_static_str(s: &str) -> &'static str {
	unsafe { &*(s as *const str) }
}

macro_rules! unsafe_obfstr {
	($string:literal) => {{
		crate::base::unsafe_as_static_str(obfstr::obfstr!($string))
	}};
}

macro_rules! strpool {
	($ctx:expr, $needle:expr) => {{
		let range = obfstr::position!($crate::STRING_POOL, $needle);
		unsafe { $ctx.strings.get_unchecked(range) }
	}};
}

// Randomize the hash function used for string lookups
pub const SEED: u32 = obfstr::random!(u32);

#[inline]
pub const fn hash(s: &str) -> u32 {
	obfstr::murmur3(s.as_bytes(), SEED)
}
macro_rules! hash {
	($s:expr) => { ::obfstr::murmur3!($s.as_bytes(), crate::base::SEED) };
}

pub mod solver;
pub mod pid;
pub mod math;
mod timer;
mod pool;

pub use self::timer::Timer;
pub use self::pool::StringPool;

#[inline(never)]
pub fn from_utf8_buf(bytes: &[u8]) -> Option<&str> {
	let len = bytes.iter().position(|&byte| byte == 0).unwrap_or(bytes.len());
	std::str::from_utf8(&bytes[..len]).ok()
}

pub fn parse_u32(value: &str) -> u32 {
	(if value.starts_with(obfstr::obfstr!("0x")) {
		let Some(src) = value.get(2..) else { return 0 };
		u32::from_str_radix(src, 16)
	}
	else {
		u32::from_str_radix(value, 10)
	}).unwrap_or(0)
}
