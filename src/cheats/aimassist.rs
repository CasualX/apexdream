use std::fmt;
use crate::*;
use crate::base::solver::{LinearPredictor, solve, TargetPredictor};
use crate::base::pid::{PidConfig, PidController};

/// Target prioritization.
///
/// Targets with a higher rank always have priority over lower ranked targets.
/// Targets with the same rank are prioritized based on their priority.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(i32)]
pub enum Rank {
	Low = -1,
	Normal = 0,
}
impl Default for Rank {
	fn default() -> Rank { Rank::Normal }
}
impl fmt::Debug for Rank {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmtools::write!(f, match self { Rank::Low => "Low", Rank::Normal => "Normal" })
	}
}

/// Target aim information.
#[derive(Copy, Clone, Debug, Default)]
pub struct TargetInfo {
	// Filled by hitpoint
	pub origin: [f32; 3],
	pub velocity: [f32; 3],
	pub spine: [[f32; 3]; 2],
	pub bone_head: i32,
	pub bone_body: i32,
	// Filled by compute
	pub hit: [f32; 3],
	pub aim: [f32; 3],
	pub distance: f32,
	pub time: f32,
	// Filled by fov_check
	pub angle: f32,
	pub pitch: f32,
	pub yaw: f32,
	// Filled by priority
	pub priority: f32,
	// Filled by rules
	pub rank: Rank,
}

struct Config {
	tickrate: f32,
	enable: bool,
	aim_team: bool,
	aim_visible: bool,
	aim_auto: bool,
	aim_camera: bool, // true = use camera angles, false = use weapon punch angles
	aim_key: i32,
	aim_pitch: f32,
	aim_yaw: f32,
	aim_ramp: f32,
	aim_fade: f32,
	aim_react: f32,
	aim_idletime: f32,
	aim_mintime: f32,
	fov_radius: f32,
	fov_aim: f32,
	fov_min: f32,
	teledist: f32,
	pid_aim: bool,
	pid: PidConfig,
	trigger_enable: bool,
	trigger_radius: f32,
	trigger_radscale: f32,
	trigger_rdyfudge: f32,
	trigger_release: f32,
	trigger_again: f32,
	trigger_react: f32,
}
impl Default for Config {
	fn default() -> Config {
		Config {
			tickrate: 144.0,
			enable: true,
			aim_team: false,
			aim_visible: true,
			aim_auto: false,
			aim_camera: false,
			aim_key: sdk::MOUSE_4,
			aim_pitch: 0.5,
			aim_yaw: 1.0,
			aim_ramp: 0.3,
			aim_fade: 0.2,
			aim_react: 0.10,
			aim_idletime: 0.3,
			aim_mintime: 0.3,
			fov_radius: 40.0,
			fov_aim: 10.0,
			fov_min: 0.25,
			teledist: 72.0,
			pid_aim: true,
			pid: PidConfig {
				kp: 1.5,
				ki: 10.0,
				kd: 0.0,
				damp: 0.9,
			},
			trigger_enable: false,
			trigger_radius: 8.0,
			trigger_radscale: 0.8,
			trigger_rdyfudge: 0.0,
			trigger_release: 0.08,
			trigger_again: 0.16,
			trigger_react: 0.1,
		}
	}
}

#[derive(Default)]
pub struct AimAssist {
	timer: base::Timer,

	// Aiming state
	addx: f32,
	addy: f32,
	pidx: PidController,
	pidy: PidController,

	// Modulate the aim strength
	aim_mod: f32,

	// Lose tracking temporarily with single fire weapons
	attack_time: f64,
	attack_state: bool,

	// Triggerbot state
	trigger_release: f64,
	trigger_again: f64,

	// Keep track of the current target
	target_locked: bool,
	target_team: bool,
	target_rank: Rank,
	target_entity: sdk::EHandle,
	target_start: f64,
	target_end: f64,
	target_yaw: f32,
	target_pos: [f32; 3],
	target_react: f64,

	config: Config,
}

impl AimAssist {
	fn is_auto_aim(&self, state: &GameState) -> bool {
		if !self.config.aim_auto {
			return false;
		}

		if !state.in_attack_down() {
			return false;
		}

		let Some(local) = state.local_player() else { return false };
		let Some(weapon) = local.active_weapon(state) else { return false };

		match weapon.weapon_name {
			sdk::WeaponName::R301 => true,
			sdk::WeaponName::ALTERNATOR => true,
			sdk::WeaponName::RE45 => true,
			sdk::WeaponName::DEVOTION => true,
			sdk::WeaponName::HAVOC => true,
			sdk::WeaponName::FLATLINE => true,
			sdk::WeaponName::HEMLOK => true,
			sdk::WeaponName::LSTAR => true,
			sdk::WeaponName::PROWLER => true,
			sdk::WeaponName::R99 => true,
			sdk::WeaponName::SPITFIRE => true,
			sdk::WeaponName::VOLT => true,
			sdk::WeaponName::RAMPAGE => true,
			sdk::WeaponName::CAR => true,
			sdk::WeaponName::EMPLACED_MINIGUN => true,
			_ => return false,
		}
	}
	#[inline(never)]
	pub fn run(&mut self, api: &mut Api, ctx: &mut RunContext) {
		let Some(local) = ctx.state.local_player() else { return };

		let enable = self.config.enable && (ctx.state.is_button_down(self.config.aim_key) || self.is_auto_aim(ctx.state));
		let trigger_type = local.active_weapon(ctx.state).map(|weapon| ctx.state.weapon_is_charged(weapon.weapon_name_index)).unwrap_or(false);

		// Trigger maintenance
		if self.config.trigger_enable {
			ctx.attack.force();

			// If trigger with BOCEK hold down attack
			if enable && trigger_type {
				ctx.attack = InState::Press;
			}
		}

		// Trigger release logic
		if self.trigger_release > 0.0 {
			ctx.attack.force();
			if ctx.state.time >= self.trigger_release {
				self.trigger_release = 0.0;
			}
			else {
				ctx.attack = if trigger_type { InState::Release } else { InState::Press };
			}
		}

		// Rate limit the aimassist
		if !self.timer.has_elapsed(ctx.state.time, 1.0 / self.config.tickrate as f64) {
			return;
		}

		// Update attack state from left mouse button
		let attacking = ctx.state.in_attack_state();
		if self.attack_state && !attacking {
			self.attack_time = ctx.state.time;
		}
		self.attack_state = attacking;

		// Aim assist turned on
		if enable {
			// Don't aim when dead or downed
			if local.is_alive() && !local.is_downed() {
				self.track(api, ctx, local);
				return;
			}
		}

		// Aim assist turned off, reset target
		self.target_entity = sdk::EHandle::default();
		self.target_locked = false;
		self.pidx.reset();
		self.pidy.reset();
		self.target_yaw = 0.0;
		self.trigger_release = 0.0;
		self.trigger_again = 0.0;
	}
	fn track<'a>(&mut self, api: &mut Api, ctx: &mut RunContext<'a>, local: &'a PlayerEntity) {
		let state = ctx.state;

		// If we're the only team present on the server then target our own team
		self.target_team = true;
		for player in state.players() {
			if player.team_num != local.team_num {
				self.target_team = false;
				break;
			}
		}

		// If we're target_locked find a new target when sufficient time has passed
		let new_target = if self.target_locked {
			state.time > self.target_end + self.config.aim_idletime as f64
		}
		// If we're not target_locked find a new target if we don't already have one
		else {
			state.time > self.target_start + self.config.aim_mintime as f64
		};

		// If requested, find a new target
		if new_target || self.target_rank == Rank::Low {
			self.find_target(ctx, local);
		}

		let weapon = local.active_weapon(state);

		// If we have a target, aim at it
		let mut valid_target = false;
		if let Some(target) = state.entity(self.target_entity) {
			let mut info = TargetInfo::default();
			if self.validate(state, local, target, &mut info) && (new_target || self.keep_target(&info)) {
				self.target_rank = info.rank;
				// Recalculate aim modulation
				self.set_aim_mod(state, weapon, target);
				// Scale the attack strength with fov scale factor
				let fov_scale = self.get_fov_scale(state, local);
				self.aim(api, &info, fov_scale);
				// Trigger when close on target
				self.trigger(ctx, &info, target, local);
				self.debug(api, &info, local);
				// Update the target time as long as it's valid
				self.target_end = state.time;
				self.target_yaw = info.yaw;
				self.target_pos = info.hit;
				self.target_rank = info.rank;
				valid_target = true;
			}
		}

		// Target is no longer valid, drop it and keep the target_locked flag
		if !valid_target {
			self.target_entity = sdk::EHandle::default();
		}
	}
	fn set_aim_mod(&mut self, state: &GameState, weapon: Option<&WeaponXEntity>, target: &dyn Entity) {
		// If target player makes sudden movement > 100deg then invoke reaction time
		if let EntityRef::Player(target) = target.as_ref() {
			if target.accel_angle() >= 100.0 {
				self.target_react = state.time;
			}
		}

		let mut strength = 1.0;

		if self.config.aim_react > 0.0 {
			let react = f32::min((state.time - self.target_react) as f32 / self.config.aim_react, 1.0);
			let react = sdk::smoothstep(react);
			strength = f32::min(strength, react);
		}

		if self.config.aim_ramp > 0.0 {
			let ramp = f32::min((state.time - self.target_start) as f32 / self.config.aim_ramp, 1.0);
			let ramp = sdk::smoothstep(ramp);
			strength = f32::min(strength, ramp);
		}

		if self.config.trigger_again > 0.0 {
			if let Some(local) = state.local_player() {
				if let Some(weapon) = weapon {
					let trigger = 1.0 - (weapon.next_ready_time - local.time_base + self.config.trigger_rdyfudge);
					let trigger = f32::clamp(trigger, 0.0, 1.0);
					// let trigger = f32::min((state.time - self.attack_time) as f32 / self.config.trigger_again, 1.0);
					let trigger = trigger * trigger;
					strength = f32::min(strength, trigger);
				}
			}
		}

		self.aim_mod = strength;
	}

	/// Find the target with the highest priority.
	fn find_target<'a>(&mut self, ctx: &mut RunContext<'a>, local: &'a PlayerEntity) {
		let mut target = None;
		let mut rank = Rank::Low;
		let mut priority = std::f32::MAX;
		let state = ctx.state;
		for entity in state.entities() {
			let mut info = TargetInfo::default();
			if self.validate(state, local, entity, &mut info) {
				if info.rank > rank || info.rank == rank && info.priority < priority {
					rank = info.rank;
					priority = info.priority;
					target = Some(entity);
				}
			}
		}

		// Only reset if we change our mind about the target
		if let Some(target) = target {
			let target_entity = target.get_info().handle;
			if self.target_entity != target_entity {
				self.target_start = state.time;
				self.target_entity = target_entity;
				self.target_locked = true;
				self.pidx.reset();
				self.pidy.reset();
				self.target_yaw = 0.0;
			}
		}
	}
	/// Returns if the current target has teleported.
	fn keep_target(&mut self, info: &TargetInfo) -> bool {
		sdk::dist(self.target_pos, info.hit) <= self.config.teledist
	}

	/// Given a target validates if the target is valid and computes how to aim.
	fn validate<'a>(&self, state: &'a GameState, local: &'a PlayerEntity, target: &'a dyn Entity, info: &mut TargetInfo) -> bool {
		if !self.rules(state, local, target, info) {
			return false;
		}
		if !self.hitpoint(state, local, target, info) {
			return false;
		}
		if !self.compute(state, local, target, info) {
			return false;
		}
		if !self.fov_check(state, local, target, info) {
			return false;
		}
		// Calculate the priority
		info.priority = info.angle + info.distance.ln();
		return true;
	}
	fn rules<'a>(&self, state: &'a GameState, local: &'a PlayerEntity, target: &'a dyn Entity, info: &mut TargetInfo) -> bool {
		info.rank = Rank::Normal;
		match target.as_ref() {
			EntityRef::Player(player) => {
				if std::ptr::eq(player, local) {
					return false;
				}
				// If the target is a player on our own team ignore them
				if state.is_same_team(local, player) {
					if !(self.config.aim_team || self.target_team) {
						return false;
					}
				}
				// If the target is not alive ignore them
				if !player.is_alive() {
					return false;
				}
				// If the target is not visible ignore them
				if self.config.aim_visible && !self.is_visible(state, player.last_visible_time) {
					return false;
				}
				// Bleeding out targets get a low priority
				if player.is_downed() {
					info.rank = Rank::Low;
				}
				return true;
			},
			EntityRef::BaseNPC(base_npc) => {
				// If the target is not visible ignore them
				if self.config.aim_visible && !self.is_visible(state, base_npc.last_visible_time) {
					return false;
				}
				info.rank = Rank::Low;
				return true;
			},
			EntityRef::Vehicle(player_vehicle) => {
				// If driven by teammate, do not shoot it
				if let Some(vehicle_driver) = state.entity_as::<PlayerEntity>(player_vehicle.vehicle_driver) {
					if vehicle_driver.team_num == local.team_num {
						if std::ptr::eq(vehicle_driver, local) || !(self.config.aim_team || self.target_team) {
							return false;
						}
					}
				}
				info.rank = Rank::Low;
				return true;
			},
			EntityRef::Animating(anim) => {
				match anim.model_name.hash {
					sdk::ModelName::SHOOTING_RANGE_TARGET => (),
					sdk::ModelName::PARIAH_DRONE => (),
					_ => return false,
				}
				info.rank = Rank::Low;
				return true;
			},
			_ => return false,
		}
	}
	fn is_visible(&self, state: &GameState, last_visible_time: f32) -> bool {
		if last_visible_time < 0.0 {
			return false;
		}
		return (last_visible_time - state.client.curtime).abs() < self.config.aim_fade;
	}
	/// Computes the hitpoint desired to hit
	fn hitpoint<'a>(&self, _state: &'a GameState, _local: &'a PlayerEntity, target: &'a dyn Entity, info: &mut TargetInfo) -> bool {
		info.origin = [0.0; 3];
		info.spine = [[0.0; 3]; 2];
		info.velocity = [0.0; 3];

		match target.as_ref() {
			EntityRef::Player(player) => {
				info.origin = player.origin;
				info.bone_head = player.studio.bone_head;
				info.bone_body = player.studio.bone_body;
				let top = player.bones.get_pos(player.studio.bone_head as usize);
				let bottom = player.bones.get_pos(player.studio.bone_body as usize);
				info.spine = [top, bottom];
				info.velocity = player.velocity;
			},
			EntityRef::BaseNPC(npc) => {
				info.origin = npc.origin;
				info.bone_head = npc.studio.bone_head;
				info.bone_body = npc.studio.bone_body;
				let top = npc.bones.get_pos(npc.studio.bone_head as usize);
				let bottom = npc.bones.get_pos(npc.studio.bone_body as usize);
				info.spine = [top, bottom];
			},
			EntityRef::Vehicle(vehicle) => {
				info.origin = vehicle.origin;
				info.velocity = vehicle.vehicle_velocity;
			},
			EntityRef::Animating(anim) => {
				info.origin = anim.origin;
				match anim.model_name.hash {
					sdk::ModelName::PARIAH_DRONE => info.origin[2] += 23.0,
					_ => (),
				}
			},
			_ => return false,
		}
		return true;
	}
	/// Computes the hit position and the angle required to aim to hit that position.
	fn compute<'a>(&self, state: &'a GameState, local: &'a PlayerEntity, _target: &'a dyn Entity, info: &mut TargetInfo) -> bool {
		// Cache the distance to the target origin
		info.distance = sdk::dist(local.view_origin, info.origin);

		// Projectile aim calculations if the weapon has projectile speed
		if let Some(weapon) = local.active_weapon(state) {
			if weapon.projectile_speed > 1.0 {
				return self.solve(local, weapon, info).is_some();
			}
		}

		// Hitscan weapons and others fall back to aiming at the target.
		info.time = 0.0;
		info.hit = sdk::add(info.origin, info.spine[0]);
		info.aim = sdk::qnorm(sdk::qangle(sdk::sub(info.hit, local.view_origin)));
		return true;
	}
	fn solve<'a>(&self, local: &'a PlayerEntity, weapon: &'a WeaponXEntity, info: &mut TargetInfo) -> Option<()> {
		// Run two predictions, one for each spine hitspot
		// This is overkill but right now I don't care
		let predictor1 = LinearPredictor {
			origin: sdk::add(info.origin, info.spine[0]),
			velocity: info.velocity,
		};
		let predictor2 = LinearPredictor {
			origin: sdk::add(info.origin, info.spine[1]),
			velocity: info.velocity,
		};
		let mut sol1 = solve(&local.view_origin, weapon, &predictor1)?;
		let mut sol2 = solve(&local.view_origin, weapon, &predictor2)?;

		// Sort the solutions ensuring sol1 is above sol2
		if sol1.pitch < sol2.pitch {
			std::mem::swap(&mut sol1, &mut sol2);
		}

		let hit1 = predictor1.predict_position(sol1.time);
		let hit2 = predictor2.predict_position(sol2.time);
		let pitch1 = -sol1.pitch.to_degrees();
		let yaw1 = sol1.yaw.to_degrees();
		let pitch2 = -sol2.pitch.to_degrees();
		let yaw2 = sol2.yaw.to_degrees();

		let local_angles = self.get_angles(local);

		// If aiming above spine[0] aim at spine[0]
		if local_angles[0] < pitch1 || pitch2 - pitch1 < 0.1 {
			info.time = sol1.time;
			info.hit = hit1;
			info.aim = [pitch1, yaw1, 0.0];
		}
		// If aiming below spine[1] aim at spine[1]
		else if local_angles[0] > pitch2 {
			info.time = sol2.time;
			info.hit = hit2;
			info.aim = [pitch2, yaw2, 0.0];
		}
		// Otherwise interpolate between the spine endpoints
		else {
			let ratio = (local_angles[0] - pitch1) / (pitch2 - pitch1);
			info.time = sol1.time + (sol2.time - sol1.time) * ratio;
			info.hit = sdk::lerp(hit1, hit2, ratio);
			let mut dyaw = yaw2 - yaw1;
			if dyaw > 180.0 { dyaw -= 360.0; }
			else if dyaw < -180.0 { dyaw += 360.0; }
			info.aim = [local_angles[0], yaw1 + dyaw * ratio, 0.0];
		}
		return Some(());
	}
	/// Checks if the target is within our FOV setting.
	fn fov_check<'a>(&self, state: &'a GameState, local: &'a PlayerEntity, target: &'a dyn Entity, info: &mut TargetInfo) -> bool {
		let local_angles = self.get_angles(local);
		info.pitch = info.aim[0] - local_angles[0];
		info.yaw = info.aim[1] - local_angles[1];
		while info.yaw > 180.0 { info.yaw -= 360.0; }
		while info.yaw <= -180.0 { info.yaw += 360.0; }
		info.angle = (info.yaw * info.yaw + info.pitch * info.pitch).sqrt();

		// Ignore FOV check when aiming at the current target
		if self.target_entity == target.get_info().handle {
			return true;
		}

		let fov_aim = self.config.fov_aim * self.get_fov_scale(state, local);
		let fov = if info.distance < 10.0 { fov_aim }
		else { f32::min(fov_aim, f32::atan(self.config.fov_radius / info.distance).to_degrees()) };

		return info.angle < fov;
	}
	fn get_angles(&self, local: &PlayerEntity) -> [f32; 3] {
		if self.config.aim_camera {
			local.camera_angles
		}
		else {
			sdk::add(local.view_angles, local.weapon_punch)
		}
	}
	fn get_fov_scale<'a>(&self, state: &'a GameState, local: &'a PlayerEntity) -> f32 {
		if local.zooming {
			if let Some(weapon) = local.active_weapon(state) {
				if weapon.target_zoom_fov > 1.0 && weapon.target_zoom_fov <= 90.0 {
					return f32::tan(weapon.target_zoom_fov.to_radians() * (90.0/70.0/2.0));
				}
			}
		}
		return 1.0;
	}

	/// Moves the mouse towards target info.
	fn aim(&mut self, api: &mut Api, info: &TargetInfo, fov_scale: f32) {
		let (dx, dy) = {
			// Avoid aim jitter with a minimum angle
			let (mut yaw, mut pitch) = (info.yaw, info.pitch);
			if info.angle < self.config.fov_min * fov_scale {
				yaw = 0.0;
				pitch = 0.0;
			}

			// Modulate the aim strength
			let strength = self.aim_mod / fov_scale;
			yaw *= strength * self.config.aim_yaw;
			pitch *= strength * self.config.aim_pitch;

			let dt = 1.0 / self.config.tickrate;
			let dx = -self.pidx.step(yaw, dt, &self.config.pid);
			let dy = self.pidy.step(pitch, dt, &self.config.pid);

			(dx, dy)
		};
		let dx = dx + self.addx;
		let dy = dy + self.addy;
		let mdx = dx as i32;
		let mdy = dy as i32;
		self.addx = dx - mdx as f32;
		self.addy = dy - mdy as f32;
		if mdx != 0 || mdy != 0 {
			api.mouse_move(mdx, mdy);
		}
	}
	/// Fires the gun if the aim is on target.
	fn trigger(&mut self, ctx: &mut RunContext, info: &TargetInfo, target: &dyn Entity, local: &PlayerEntity) {
		if !self.config.trigger_enable {
			return;
		}
		if ctx.state.time < self.trigger_again {
			return;
		}

		let Some(weapon) = local.active_weapon(ctx.state) else { return };
		match weapon.weapon_name {
			sdk::WeaponName::BOCEK => {
				// Trigger Bocek only on full strength
				if weapon.projectile_scale != 1.5 {
					return;
				}
			},
			sdk::WeaponName::THROWING_KNIFE => {
				// Trigger throwing knife when held
				if weapon.weap_state.0 != 18 {
					return;
				}
			},
			_ => {
				// Do not trigger if the player is attacking
				if ctx.state.in_attack_down() {
					return;
				}
			},
		}

		let hit = match target.as_ref() {
			EntityRef::Player(target) => {
				if ctx.state.time < target.visible_time + self.config.trigger_react as f64 {
					return;
				}
				is_hit(&local.view_origin, &local.camera_angles, &target.origin, &target.bones.v, &target.studio, self.config.trigger_radscale)
			},
			EntityRef::BaseNPC(target) => {
				if ctx.state.time < target.visible_time + self.config.trigger_react as f64 {
					return;
				}
				is_hit(&local.view_origin, &local.camera_angles, &target.origin, &target.bones.v, &target.studio, self.config.trigger_radscale)
			},
			_ => {
				let angle = f32::atan(self.config.trigger_radius / info.distance).to_degrees();
				(info.yaw * info.yaw + info.pitch * info.pitch) < angle * angle
			},
		};

		if hit {
			if weapon.next_ready_time == 0.0 && weapon.burst_fire_index == 0 {
				ctx.attack = InState::Press;
				self.trigger_release = ctx.state.time + self.config.trigger_release as f64;
				self.trigger_again = ctx.state.time + self.config.trigger_again as f64;
			}
		}
	}
	fn debug(&self, api: &mut Api, info: &TargetInfo, _local: &PlayerEntity) {
		api.visualize(s!("AimAssist"), xfmt! {
			<h2>"AimAssist"</h2>
			<pre>
				"PidX: "{self.pidx:#?}"\n"
				"PidY: "{self.pidy:#?}"\n"
				"Origin:   "{info.origin;?}"\n"
				"Velocity: "{info.velocity;?}"\n"
				"Spine[0]: "{info.spine[0];?}"\n"
				"Spine[1]: "{info.spine[1];?}"\n"
				"BoneHead: "{info.bone_head}"\n"
				"BoneBody: "{info.bone_body}"\n"
				"Hit:      "{info.hit;?}"\n"
				"Aim:      "{info.aim;?}"\n"
				"Distance: "{info.distance;?}"\n"
				"Time:     "{info.time;?}"\n"
				"Angle:    "{info.angle;?}"\n"
				"Pitch:    "{info.pitch;?}"\n"
				"Yaw:      "{info.yaw;?}"\n"
				"Priority: "{info.priority;?}"\n"
				"Rank:     "{info.rank;?}"\n"
			</pre>
		});
	}
}

fn is_hit(origin: &[f32; 3], angles: &[f32; 3], bone_origin: &[f32; 3], bones: &[[f32; 12]], studio: &state::StudioModel, radius_scale: f32) -> bool {
	let d = sdk::qvec(*angles);

	let mut data = Vec::new();
	for bbox in studio.spine() {
		data.push((bbox.bone, bbox.radius()));
	}

	for i in 0..100 {
		let fi = i as i32 as f32 / 100.0 * data.len() as i32 as f32;
		let starti = fi.floor() as i32 as usize;
		let endi = fi.ceil() as i32 as usize;
		let t = fi.fract();

		if starti >= data.len() || endi >= data.len() {
			break;
		}

		let start = match bones.get(data[starti].0 as usize) { Some(start) => start, None => continue };
		let end = match bones.get(data[endi].0 as usize) { Some(end) => end, None => continue };

		let start = sdk::add(*bone_origin, [start[3], start[7], start[11]]);
		let end = sdk::add(*bone_origin, [end[3], end[7], end[11]]);

		let bone_pos = sdk::lerp(start, end, t);
		let radius = (data[starti].1 + (data[endi].1 - data[starti].1) * t) * radius_scale;

		let dist2 = sdk::dist2(sdk::project(*origin, d, bone_pos), bone_pos);
		if dist2 < radius * radius {
			return true;
		}
	}

	return false;
}
