use super::*;

struct Config {
	enable: bool,
	filter: bool,
	debug: bool,
	pitch: f32,
	yaw: f32,
}
impl Default for Config {
	fn default() -> Self {
		Config {
			enable: true,
			filter: true,
			debug: false,
			pitch: 0.5,
			yaw: 0.0,
		}
	}
}

#[derive(Default)]
pub struct RCS {
	timer: base::Timer,
	config: Config,
	attacking: bool,
	sens: f32,
	old_punch: [f32; 3],
	my: f32,
	mx: f32,
}

impl cvar::IVisit for RCS {
	fn visit(&mut self, f: &mut dyn FnMut(&mut dyn cvar::INode)) {
		let default = Config::default();
		f(&mut cvar::Property(s!("enable"), &mut self.config.enable, &default.enable));
		f(&mut cvar::Property(s!("filter"), &mut self.config.filter, &default.filter));
		f(&mut cvar::Property(s!("debug"), &mut self.config.debug, &default.debug));
		f(&mut cvar::Property(s!("pitch"), &mut self.config.pitch, &default.pitch));
		f(&mut cvar::Property(s!("yaw"), &mut self.config.yaw, &default.yaw));
	}
}

impl RCS {
	#[inline(never)]
	pub fn run(&mut self, api: &mut Api, ctx: &RunContext) {
		self.attacking = self.work(api, ctx);
	}

	fn filter(&self, weapon: &WeaponXEntity) -> bool {
		// if weapon.is_semi_auto {
		// 	return false;
		// }
		use sdk::WeaponName as WN;
		match weapon.weapon_name {
			WN::R301 => true,
			WN::ALTERNATOR => true,
			WN::RE45 => true,
			WN::DEVOTION => true,
			WN::HAVOC => true,
			WN::FLATLINE => true,
			WN::G7_SCOUT => true,
			WN::HEMLOK => true,
			WN::LSTAR => true,
			WN::PROWLER => true,
			WN::R99 => true,
			WN::P2020 => true,
			WN::SPITFIRE => true,
			WN::VOLT => true,
			WN::RAMPAGE => true,
			WN::CAR => true,
			WN::NEMESIS => true,
			WN::EMPLACED_MINIGUN => true,
			_ => false,
		}
	}

	fn work(&mut self, api: &mut Api, ctx: &RunContext) -> bool {
		if !self.config.enable {
			return false;
		}
		let Some(local) = ctx.state.local_player() else {
			return false;
		};
		let Some(weapon) = local.active_weapon(ctx.state) else {
			return false;
		};
		if self.config.filter && !self.filter(weapon) {
			return false;
		}

		// Run RCS only when the player is attacking or when burst firing
		let attacking = ctx.state.in_attack_state() || weapon.burst_fire_index != 0;
		if !attacking {
			return false;
		}

		// Weapon punch includes things like:
		// * Recoil from the weapon
		// * Recoil from landing on the ground
		// * Recoil from climbing and vaulting
		//
		// Isolate the weapon recoil by lookin when recoil pitch is negative
		// Other forms of recoil tend to have positive recoil pitch
		let mut weapon_punch = local.weapon_punch;
		if weapon_punch[0] > 0.0 {
			weapon_punch = [0.0; 3];
		}

		// Initialize RCS when starting to attack
		if !self.attacking {
			self.my = 0.0;
			self.mx = 0.0;
			self.old_punch = weapon_punch;

			let _ = api.vm_read_into(ctx.process.base.field(ctx.data.mouse_sensitivity + 0x68), &mut self.sens);
		}
		else {
			// Pitch RCS only when it is increasing
			// It's unclear why this is so much more effective
			let pitch = f32::max(0.0, self.old_punch[0] - weapon_punch[0]);
			let yaw = weapon_punch[1] - self.old_punch[1];
			self.old_punch = weapon_punch;

			// Send the compensated punch angles as mouse movements
			self.aim(api, ctx, pitch, yaw);
		}

		return true;
	}

	fn aim(&mut self, api: &mut Api, ctx: &RunContext, pitch: f32, yaw: f32) {
		// Modulate the intensity by the user's sensitivity stuff
		let scale = self.sens * (1.0 / 0.022);

		// Accumulate aiming deltas
		self.my += pitch * self.config.pitch * scale;
		self.mx += yaw * self.config.yaw * scale;

		if !self.config.debug {
			// Mouse movement is quantized and keep track of risiduals
			let my = self.my.round();
			let mx = self.mx.round();
			self.my -= my;
			self.mx -= mx;

			let my = my as i32;
			let mx = mx as i32;
			if mx != 0 || my != 0 {
				api.mouse_move(mx, my);
			}
		}
		else if self.timer.has_elapsed(ctx.state.time, 0.025) {
			api.visualize(s!("RCS"), xfmt!{
				<pre>
					"sens:  "{self.sens}"\n"
					"punch: "{self.old_punch:?}"\n"
					"my:    "{self.my}"\n"
					"mx:    "{self.mx}"\n"
				</pre>
			});
		}
	}
}
