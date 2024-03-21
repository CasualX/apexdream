use crate::*;

pub struct Config {
	pub enable: bool,
	pub lock: bool,
	pub team: bool,
	pub centerx: i32,
	pub centery: i32,
	pub scale: f32,
	pub clamp: bool,
	pub radius: f32,
}
impl Default for Config {
	fn default() -> Config {
		Config {
			enable: true,
			lock: false,
			team: false,
			centerx: 0,
			centery: 0,
			scale: 0.05,
			clamp: true,
			radius: 8000.0,
		}
	}
}
impl Config {
	fn distance_alpha(&self, d: f32) -> f32 {
		(1.0 - (d - self.radius).max(0.0) / (self.radius * 0.5)).max(0.0)
	}
}

#[derive(Default)]
pub struct Radar {
	config: Config,
}


impl Radar {
	#[inline(never)]
	pub fn render(&mut self, api: &mut Api, ctx: &RunContext) {
		if !self.config.enable {
			return;
		}

		let Some(local) = ctx.state.local_player() else { return };

		for entity in ctx.state.entities() {
			match entity.as_ref() {
				EntityRef::Player(player) => {
					// For radar don't show downed players
					if !player.is_alive() || player.is_downed() {
						continue;
					}
					// If player is on the same team then only show if enabled
					if !self.config.team && ctx.state.is_same_team(player, local) {
						continue;
					}
					let ([px, py], d) = self.transform(ctx, local, &player.origin);
					let alpha = self.config.distance_alpha(d);
					if alpha > 0.0 {
						let alpha = (alpha * 255.0) as u8;
						api.r_rect(
							/*x:*/ px - 2.0,
							/*y:*/ py - 2.0,
							/*width:*/ 4.0,
							/*height:*/ 4.0,
							/*fill:*/ vgc::sRGB::from(player.team_color).alpha(alpha),
							/*stroke:*/ vgc::sRGBA!(Black, alpha),
						);
					}
				},
				_ => (),
			}
		}
	}
	fn transform(&self, ctx: &RunContext, local: &PlayerEntity, origin: &[f32; 3]) -> ([f32; 2], f32) {
		// Use camera origin and angles if local is real local player
		let (local_origin, local_angles) = (&local.view_origin, &local.camera_angles);

		// Translate around local origin
		let mut x = origin[0] - local_origin[0];
		let mut y = origin[1] - local_origin[1];

		// Rotate around local origin
		if !self.config.lock {
			let yaw = local_angles[1].to_radians();
			let (sin, cos) = yaw.sin_cos();
			let nx = y * -cos + x * sin;
			let ny = x * -cos - y * sin;
			x = nx;
			y = ny;
		}

		// Clamp to circle around the crosshair
		let d = (x * x + y * y).sqrt();
		if self.config.clamp {
			if d > self.config.radius {
				let radius = self.config.radius / d;
				x *= radius;
				y *= radius;
			}
		}

		// Translate to pixel coordinates
		let centerx = (ctx.screen[0] >> 1) + self.config.centerx;
		let centery = (ctx.screen[1] >> 1) + self.config.centery;
		let px = (x * self.config.scale + centerx as f32).round();
		let py = (y * self.config.scale + centery as f32).round();

		([px, py], d)
	}
}
