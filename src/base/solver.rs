/*!
Projectile solver.
!*/

fn avg(a: usize, b: usize) -> usize {
	a / 2 + b / 2 + (a % 2 + b % 2) / 2
}

#[derive(Copy, Clone, Debug)]
pub struct TimeAngle {
	pub time: f32,
	pub pitch: f32,
}

/// Prerecorded trajectory.
#[derive(Copy, Clone, Debug)]
pub struct Trajectory<'a> {
	/// The angle the projectile was fired at.
	pub pitch: f32,
	/// The samples are taken at even time intervals.
	pub timestep: f32,
	/// The sampled trajectory coordinates.
	///
	/// The samples must have strictly increasing distance to the origin.
	pub samples: &'a [[f32; 2]],
}

impl<'a> Trajectory<'a> {
	/// Finds the intersection between this trajectory and a circle with radius `dist`.
	///
	/// Returns the point of intersection as the time when the projectile reaches it and the angle it makes with the origin.
	pub fn intersect(&self, dist: f32) -> Option<TimeAngle> {
		let dist2 = dist * dist;

		// Reject distances larger than our max range
		let last = self.samples.last()?;
		if !(dist2 < last[0] * last[0] + last[1] * last[1]) {
			return None;
		}

		// Binary search for the intersection between the trajectory and the distance to [x, y]
		let mut low = 0;
		let mut high = self.samples.len() - 1;
		while low + 1 != high {
			let middle = avg(low, high);
			let &[x, y] = self.samples.get(middle)?;

			if dist2 < x * x + y * y {
				high = middle;
			}
			else {
				low = middle;
			}
		}

		// Intersect the circle with the trajectory
		let &[low_x, low_y] = self.samples.get(low)?;
		let &[high_x, high_y] = self.samples.get(high)?;
		let low_dist = f32::sqrt(low_x * low_x + low_y * low_y);
		let high_dist = f32::sqrt(high_x * high_x + high_y * high_y);
		let fract = (dist - low_dist) / (high_dist - low_dist);

		// Linear interpolate between the two samples for more precision
		let time = (low as i32 as f32 + fract) * self.timestep;
		// let x = low_x + fract * (high_x - low_x);
		let y = low_y + fract * (high_y - low_y);
		let pitch = f32::asin(y / dist);

		Some(TimeAngle { time, pitch })
	}
}

/// Collection of prerecorded trajectories sorted by increasing pitch.
#[derive(Copy, Clone, Debug)]
pub struct Collection<'a> {
	pub collection: &'a [Trajectory<'a>],
}
#[allow(non_snake_case)]
pub const fn Collection<'a>(collection: &'a [Trajectory<'a>]) -> Collection<'a> {
	Collection { collection }
}

impl<'a> Collection<'a> {
	/// Plan a trajectory to hit the target at given coordinates launched from the origin.
	pub fn plan(&self, [x, y]: [f32; 2]) -> Option<TimeAngle> {
		let collection = self.collection;

		let lowest = collection.first()?;
		let highest = collection.last()?;

		// Precalculate some values we'll need later
		let target_dist = f32::sqrt(x * x + y * y);
		if !(target_dist >= 0.1) {
			return None;
		}
		let target_angle = f32::asin(y / target_dist);

		// TODO! Extrapolate from lowest or highest trajectories instead of outright rejecting
		if target_angle < lowest.pitch || target_angle > highest.pitch {
			return None;
		}

		// Binary search for the trajectories below and above where we'll need to aim to hit the target
		let mut low = 0;
		let mut high = collection.len() - 1;
		while low + 1 != high {
			let middle = avg(low, high);
			let trajectory = collection.get(middle)?;
			let ta = trajectory.intersect(target_dist)?;

			if target_angle < ta.pitch {
				high = middle;
			}
			else {
				low = middle;
			}
		}

		// Fetch and intersect the trajectories again
		let low_t = collection.get(low)?;
		let high_t = collection.get(high)?;

		let low_ta = low_t.intersect(target_dist)?;
		let high_ta = high_t.intersect(target_dist)?;

		// Based on how close the pitch angle is to either trajectory
		let fract = (target_angle - low_ta.pitch) / (high_ta.pitch - low_ta.pitch);

		// Linear interpolate between the trajectories
		let time = low_ta.time + fract * (high_ta.time - low_ta.time);
		let pitch = low_t.pitch + fract * (high_t.pitch - low_t.pitch);

		return Some(TimeAngle { time, pitch });
	}
}

pub trait TargetPredictor {
	fn predict_position(&self, time: f32) -> [f32; 3];
}
pub trait ProjectileWeapon {
	fn projectile_speed(&self) -> f32;
	fn projectile_gravity(&self) -> f32;
	fn projectile_fire_setup(&self, origin: &[f32; 3], target: &[f32; 3]) -> [f32; 3] {
		let dx = target[0] - origin[0];
		let dy = target[1] - origin[1];
		let dz = target[2] - origin[2];
		[dx, dy, dz]
	}
	fn projectile_collection(&self) -> Option<Collection> {
		None
	}
}

/// Simple linear extrapolation.
pub struct LinearPredictor {
	pub origin: [f32; 3],
	pub velocity: [f32; 3],
}
impl TargetPredictor for LinearPredictor {
	fn predict_position(&self, time: f32) -> [f32; 3] {
		let x = self.origin[0] + self.velocity[0] * time;
		let y = self.origin[1] + self.velocity[1] * time;
		let z = self.origin[2] + self.velocity[2] * time;
		[x, y, z]
	}
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Solution {
	pub pitch: f32,
	pub yaw: f32,
	pub time: f32,
}

const MAX_TIME: f32 = 1.0;
const TIME_STEP: f32 = 1.0 / 256.0;

pub fn solve(player: &[f32; 3], weapon: &dyn ProjectileWeapon, target: &dyn TargetPredictor) -> Option<Solution> {
	let mut target_time = 0.0;
	while target_time <= MAX_TIME {
		let target_pos = target.predict_position(target_time);
		let sol = solve2d(player, weapon, &target_pos)?;
		if sol.time < target_time {
			return Some(sol);
		}
		target_time += TIME_STEP;
	}
	None
}
fn solve2d(player: &[f32; 3], weapon: &dyn ProjectileWeapon, target: &[f32; 3]) -> Option<Solution> {
	let v = weapon.projectile_fire_setup(player, target);
	let dx = f32::sqrt(v[0] * v[0] + v[1] * v[1]);
	let dy = v[2];
	if let Some(collection) = weapon.projectile_collection() {
		collection.plan([dx, dy]).map(|TimeAngle { time, pitch: angle } | {
			let yaw = f32::atan2(v[1], v[0]);
			Solution { pitch: angle, yaw, time }
		})
	}
	else {
		let v0 = weapon.projectile_speed();
		let g = weapon.projectile_gravity();
		let pitch = optimal(dx, dy, v0, g)?;
		let time = dx / (f32::cos(pitch) * v0);
		let yaw = f32::atan2(v[1], v[0]);
		Some(Solution { pitch, yaw, time })
	}
}
fn optimal(x: f32, y: f32, v0: f32, g: f32) -> Option<f32> {
	let root = v0 * v0 * v0 * v0 - g * (g * x * x + 2.0 * y * v0 * v0);
	if root < 0.0 {
		return None;
	}
	let root = f32::sqrt(root);
	let angle = f32::atan((v0 * v0 - root) / (g * x));
	// let time = x / (f32::cos(angle) * v0);
	Some(angle)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn units() {
		struct Player([f32; 3]);
		struct Weapon(f32, f32);
		impl ProjectileWeapon for Weapon {
			fn projectile_speed(&self) -> f32 { self.0 }
			fn projectile_gravity(&self) -> f32 { self.1 }
		}
		impl TargetPredictor for Player {
			fn predict_position(&self, _time: f32) -> [f32; 3] { self.0 }
		}

		let target = Player([1000.0, 2000.0, 0.0]);
		let player = Player([500.0, 100.0, 100.0]);
		let weapon = Weapon(10000.0, 750.0);

		let sol = solve(&player.0, &weapon, &target);
		panic!("{:?}", sol);
	}
}
