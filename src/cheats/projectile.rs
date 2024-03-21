use crate::*;

#[derive(Copy, Clone, Debug)]
struct Record {
	time: f64,
	origin: [f32; 3],
	velocity: [f32; 3],
}

#[derive(Default)]
pub struct Projectile {
	enable: bool,
	in_attack: bool,
	handle: sdk::EHandle,
	history: Vec<Record>,
	launch_time: f64,
	launch_origin: [f32; 3],
	launch_pitch: f32,
	view_origin: [f32; 3],
	view_angles: [f32; 3],
	aim_vector: [f32; 3],
	projectile_speed: f32,
	projectile_scale: f32,
}



impl Projectile {
	pub fn run(&mut self, api: &mut Api, ctx: &mut RunContext) {
		if !self.enable {
			return;
		}

		let state = ctx.state;
		let Some(local) = state.local_player() else { return };
		let Some(weapon) = local.active_weapon(state) else { return };

		// Capture the player's camera angles when attacking
		// Avoid doing this when the projectile is created to capture the camera angles before recoil kicks in
		let in_attack = state.in_attack_down();
		if self.in_attack != in_attack && in_attack != state.weapon_is_charged(weapon.weapon_name_index) {
			self.view_origin = local.view_origin;
			self.view_angles = local.camera_angles;
			self.aim_vector = sdk::qvec(local.camera_angles);
			self.projectile_speed = weapon.projectile_speed;
			self.projectile_scale = weapon.projectile_scale;
		}
		self.in_attack = in_attack;

		// Find a projectile fired by local player to analyze
		if !self.handle.is_valid() {
			if let Some(p) = state.entities_as::<ProjectileEntity>().filter(|p| p.owner_entity == state.client.local_entity).next() {
				self.handle = p.get_info().handle;
				// The transition from client to server sided projectile keeps the same launch origin
				if self.launch_origin != p.launch_origin {
					self.history.clear();
					self.launch_time = state.time;
					self.launch_origin = p.launch_origin;
					let dx = f32::sqrt(p.velocity[0]*p.velocity[0] + p.velocity[1]*p.velocity[1]);
					self.launch_pitch = f32::atan2(p.velocity[2], dx);
				}
			}
		}

		// Look up the target projectile
		if let Some(p) = state.entity_as::<ProjectileEntity>(self.handle) {
			let time = state.time - self.launch_time;
			let origin = sdk::sub(p.origin, self.launch_origin);
			let velocity = p.velocity;

			// Wait for projectile position to change
			if let Some(last) = self.history.last() {
				if origin == last.origin {
					return;
				}
			}
			self.history.push(Record { time, origin, velocity });
		}
		// Projectile just went away and we've captured a full log
		else if self.handle.is_valid() {
			self.handle = sdk::EHandle::default();
			api.visualize(s!("Projectile"), xfmt! {
				<pre>
					"launch_origin: "{self.launch_origin;?}"\n"
					"view_origin:   "{self.view_origin;?}"\n"
					if let Some(first) = self.history.first() {
						"proj_velocity: "{first.velocity:?}"\n"
					}
					"launch_pitch:  "{-self.launch_pitch.to_degrees()}"\n"
					"Pitch { view: "{(-self.view_angles[0]).to_radians();.4}", launch: "{self.launch_pitch;.4}" },\n"
					"aim_pitch:     "{self.view_angles[0];?}"\n"
					"aim_vector:    "{self.aim_vector;?}"\n"
					"projectile_speed: "{self.projectile_speed}"\n"
					"projectile_scale: "{self.projectile_scale}"\n"
					"\n"
					"time,x,z,speed\n"
					for &Record { time, origin, velocity } in (&self.history) {
						let time = time as f32;
						let [x, z] = project(origin);
						let s = sdk::len(velocity);
						// let [px, py] = predict((-self.camera_angles[0]).to_radians(), time);
						// {time}","{x}","{z}","{s}","{px}","{py}"\n"
						{time}","{x}","{z}","{s}"\n"
					}
				</pre>
			});

			const TIMESTEP: f64 = 1.0 / 32.0;
			const MAX_TIME: f64 = 2.0;

			api.visualize(s!("Projectile2"), xfmt! {
				<pre>
				"pub const COLLECTION: Collection = Collection(&[\n"
					"\tTrajectory { pitch: "{(-self.view_angles[0]).to_radians()}", timestep: "{TIMESTEP}", samples: &["
					for (index, [x, y]) in (self.samples(TIMESTEP, MAX_TIME).enumerate()) {
						if (index % 4 == 0) {
							"\n\t\t"
						}
						"["{x}"f32,"{y}"f32],"
					}
					"\n\t]},\n"
				"]);\n"
				</pre>
			});
		}
	}
	fn samples(&self, step: f64, max: f64) -> impl '_ + Iterator<Item = [f32; 2]> {
		let mut i = 1;
		let mut time = 0.0;
		std::iter::from_fn(move || {
			if time > max {
				return None;
			}
			loop {
				let low = self.history.get(i - 1)?;
				let high = self.history.get(i)?;
				if time >= low.time && time <= high.time {
					let fract = ((time - low.time) / (high.time - low.time)) as f32;
					let [low_x, low_y] = project(low.origin);
					let [high_x, high_y] = project(high.origin);
					let x = low_x + fract * (high_x - low_x);
					let y = low_y + fract * (high_y - low_y);
					time += step;
					return Some([x, y]);
				}
				i += 1;
			}
		})
	}
}

fn project([dx, dy, dz]: [f32; 3]) -> [f32; 2] {
	[f32::sqrt(dx * dx + dy * dy), dz]
}
