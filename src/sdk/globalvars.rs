use super::Pod;

pub type CSaveRestoreData = ();

#[derive(Default, Debug)]
#[repr(C)]
pub struct CGlobalVars {
	pub realtime: f64,
	pub framecount: i32,
	pub absframetime: f32,
	// Best curtime to use: compared to last_visible_time of NPCs in firing range
	pub curtime: f32,
	pub curtime1: f32,
	pub curtime2: f32,
	pub curtime3: f32,
	pub interval_per_tick: f32,
	pub curtime4: f32, // same as curtime?
	pub curtime5: f32,
}
unsafe impl Pod for CGlobalVars {}
