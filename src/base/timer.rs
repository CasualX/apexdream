
#[derive(Copy, Clone, Debug, Default)]
pub struct Timer {
	last_time: f64,
}
impl Timer {
	pub fn has_elapsed(&mut self, time: f64, interval: f64) -> bool {
		let next_time = self.last_time + interval;
		if time >= next_time {
			self.last_time = if time >= next_time + interval { time } else { next_time };
			true
		}
		else {
			false
		}
	}
}
