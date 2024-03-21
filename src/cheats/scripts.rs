use crate::*;

struct Config {
	rapidfire: bool,
	bunnyhop: bool,
	autoreload: bool,
	tacreload: bool,
	acreload: bool,
	tapstrafekey: i32,
	thirdperson: bool,
	thirdperson_shoulder: bool,
	freecam: bool,
	fastloot: bool,
	autoloot: i32,
	limit_fps: bool,
	superglide: bool,
}
impl Default for Config {
	fn default() -> Config {
		Config {
			rapidfire: true,
			bunnyhop: false,
			autoreload: true,
			tacreload: false,
			acreload: false,
			tapstrafekey: sdk::MOUSE_4,
			thirdperson: false,
			thirdperson_shoulder: true,
			freecam: false,
			fastloot: true,
			autoloot: sdk::MOUSE_4,
			limit_fps: false,
			superglide: false,
		}
	}
}

#[derive(Default)]
pub struct Scripts {
	config: Config,
	ts_end: f64,
	thirdperson: bool,
	freecam: bool,
	limit_fps: bool,
	// jumping: bool,
	// jumptime: f64,
	climbtime: f64,
	glidetime: f64,

	ammo_in_clip: i32,
	acreloadtime: f64,

	// Total time needed for this ready time to expire when it started
	ready_timer: f32,
}

impl Scripts {
	pub fn run(&mut self, api: &mut Api, ctx: &mut RunContext) {
		self.bunnyhop(ctx);
		self.rapidfire(ctx);
		self.autoreload(ctx);
		self.acreload(api, ctx);
		self.tapstrafe(ctx);
		self.thirdperson(api, ctx);
		self.freecam(api, ctx);
		self.loot(api, ctx);
		self.superglide(ctx);

		if ctx.data.fps_max != 0 {
			let fps_max_ptr = ctx.process.base.field::<()>(ctx.data.fps_max);
			if !self.limit_fps && self.config.limit_fps {
				let _ = api.vm_write(fps_max_ptr.field(0x68), &30.0f32);
				let _ = api.vm_write(fps_max_ptr.field(0x6c), &30i32);
			}
			if self.limit_fps && !self.config.limit_fps {
				let _ = api.vm_write(fps_max_ptr.field(0x68), &144.0f32);
				let _ = api.vm_write(fps_max_ptr.field(0x6c), &144i32);
			}
			self.limit_fps = self.config.limit_fps;
		}
	}
	fn bunnyhop(&mut self, ctx: &mut RunContext) {
		if !self.config.bunnyhop {
			return;
		}
		ctx.jump.force();
		let Some(local) = ctx.state.local_player() else { return };
		if !local.ground_entity.is_valid() {
			ctx.jump = InState::Release;
		}
	}
	fn rapidfire(&mut self, ctx: &mut RunContext) {
		if !self.config.rapidfire {
			self.ready_timer = 0.0;
			return;
		}

		ctx.attack.force();

		let state = ctx.state;
		let Some(local) = state.local_player() else { return };
		let Some(weapon) = local.active_weapon(state) else { return };

		// Rapidfire weapon whitelist
		match weapon.weapon_name {
			sdk::WeaponName::P2020 |
			sdk::WeaponName::WINGMAN |
			sdk::WeaponName::EVA8_AUTO |
			sdk::WeaponName::PEACEKEEPER |
			sdk::WeaponName::MASTIFF |
			sdk::WeaponName::R301 |
			sdk::WeaponName::FLATLINE |
			sdk::WeaponName::PROWLER |
			sdk::WeaponName::HEMLOK |
			sdk::WeaponName::G7_SCOUT |
			sdk::WeaponName::REPEATER |
			sdk::WeaponName::TRIPLE_TAKE |
			sdk::WeaponName::SENTINEL |
			sdk::WeaponName::CHARGE_RIFLE |
			sdk::WeaponName::LONGBOW => (),
			_ => return self.ready_timer = 0.0,
		}

		// Rapidfire only when semi-auto
		if !weapon.is_semi_auto {
			self.ready_timer = 0.0;
			return;
		}

		// Block attack while reloading
		if weapon.in_reload {
			self.ready_timer = 0.0;
			ctx.attack = InState::Release;
			return;
		}

		// Track how long until ready time
		let ready_time = f32::max(0.0, weapon.next_ready_time - local.time_base);
		if ready_time == 0.0 {
			self.ready_timer = 0.0;
		}
		else if self.ready_timer == 0.0 {
			self.ready_timer = ready_time;
		}

		// Legit, perfect rapidfire
		if ready_time == 0.0 || ready_time > self.ready_timer * 0.75 {
		}
		else if ready_time > f32::min(0.1, self.ready_timer * 0.1) {
			if weapon.burst_fire_count == 0 || weapon.burst_fire_index == 0 {
				ctx.attack = InState::Release;
			}
		}
	}
	fn autoreload(&mut self, ctx: &mut RunContext) {
		if !self.config.autoreload {
			return;
		}

		ctx.attack.force();
		ctx.reload.force();

		let state = ctx.state;
		let Some(local) = state.local_player() else { return };
		let Some(weapon) = local.active_weapon(state) else { return };

		if weapon.in_reload {
			return;
		}

		// Tactical reload when 1 bullet left for faster reload
		let tac_ammo = if self.config.tacreload { 1 } else { 0 };

		// Configure auto reload per weapon
		let min_ammo = match weapon.weapon_name {
			sdk::WeaponName::HAVOC => 0,
			sdk::WeaponName::FLATLINE => tac_ammo,
			sdk::WeaponName::HEMLOK => 0,
			sdk::WeaponName::R301 => tac_ammo,

			sdk::WeaponName::ALTERNATOR => tac_ammo,
			sdk::WeaponName::PROWLER => tac_ammo,
			sdk::WeaponName::R99 => tac_ammo,
			sdk::WeaponName::VOLT => tac_ammo,
			sdk::WeaponName::CAR => tac_ammo,

			sdk::WeaponName::DEVOTION => tac_ammo,
			sdk::WeaponName::LSTAR => return,
			sdk::WeaponName::SPITFIRE => tac_ammo,
			sdk::WeaponName::RAMPAGE => tac_ammo,

			sdk::WeaponName::G7_SCOUT => tac_ammo,
			sdk::WeaponName::TRIPLE_TAKE => 0,
			sdk::WeaponName::REPEATER => 0,
			sdk::WeaponName::BOCEK => return,

			sdk::WeaponName::CHARGE_RIFLE => 0,
			sdk::WeaponName::LONGBOW => 0,
			sdk::WeaponName::KRABER => 0,
			sdk::WeaponName::SENTINEL => 0,

			sdk::WeaponName::EVA8_AUTO => 0,
			sdk::WeaponName::MASTIFF => 0,
			sdk::WeaponName::MOZAMBIQUE => 0,
			sdk::WeaponName::PEACEKEEPER => 0,

			sdk::WeaponName::RE45 => tac_ammo,
			sdk::WeaponName::P2020 => 0,
			sdk::WeaponName::WINGMAN => 0,

			_ => return,
		};

		// Wait until low on ammo
		if weapon.ammo_in_clip > min_ammo {
			return;
		}

		// Do nothing if the player has no spare ammo
		let spare_ammo = local.get_consumable_count(state, weapon.ammo_item(state)) + weapon.ammo_in_stockpile;
		if spare_ammo <= 0 {
			return;
		}

		ctx.attack = InState::Release;
		ctx.reload = InState::Press;
	}
	fn acreload(&mut self, api: &mut Api, ctx: &mut RunContext) {
		if !self.config.acreload {
			return;
		}

		let state = ctx.state;
		let Some(local) = state.local_player() else { return };

		if self.acreloadtime > 0.0 {
			if state.time >= self.acreloadtime + 0.05 {
				ctx.select_slot(api, local.selected_slot);
				self.acreloadtime = 0.0;
			}
			return;
		}

		let Some(weapon) = local.active_weapon(state) else { return };
		match weapon.weapon_name {
			sdk::WeaponName::SPITFIRE => (),
			sdk::WeaponName::HAVOC => (),
			sdk::WeaponName::R301 => (),
			sdk::WeaponName::G7_SCOUT => (),
			sdk::WeaponName::PEACEKEEPER => (),

			sdk::WeaponName::FLATLINE => (),
			sdk::WeaponName::HEMLOK => (),
			sdk::WeaponName::DEVOTION => (),
			sdk::WeaponName::PROWLER => (),
			sdk::WeaponName::KRABER => (),
			sdk::WeaponName::EVA8_AUTO => (),
			sdk::WeaponName::MOZAMBIQUE => (),
			sdk::WeaponName::LONGBOW => (),
			sdk::WeaponName::TRIPLE_TAKE => (),
			sdk::WeaponName::RE45 => (),
			sdk::WeaponName::ALTERNATOR => (),

			_ => {
				self.ammo_in_clip = -1;
				self.acreloadtime = 0.0;
				return
			},
		};

		if weapon.in_reload && self.ammo_in_clip == 0 && weapon.ammo_in_clip > self.ammo_in_clip {
			ctx.select_slot(api, 2);
			self.acreloadtime = state.time;
		}
		self.ammo_in_clip = weapon.ammo_in_clip;
	}
	fn tapstrafe(&mut self, ctx: &mut RunContext) {
		if self.config.tapstrafekey <= 0 {
			return;
		}

		ctx.forward.force();

		let state = ctx.state;
		let Some(local) = state.local_player() else { return };

		// When jumping or leaving the ground enable tap-strafing for 0.5 seconds
		let is_on_ground = local.ground_entity.is_valid();
		if is_on_ground {
			self.ts_end = state.time + 0.5;
		}
		else if state.time < self.ts_end {
			if state.is_button_down(self.config.tapstrafekey) && (state.in_moveleft() || state.in_moveright()) {
				ctx.forward = if ctx.rapidfire() { InState::Press } else { InState::Release };
			}
		}
		else if state.in_jump() {
			self.ts_end = state.time + 0.5;
		}
	}
	fn thirdperson(&mut self, api: &mut Api, ctx: &mut RunContext) {
		if ctx.data.thirdperson_override == 0 {
			return;
		}
		let thirdperson_ptr = ctx.process.base.field(ctx.data.thirdperson_override + 0x6c);
		if self.config.thirdperson {
			// Crude approximation for disabling thirdperson when ADS
			let is_ads = ctx.state.in_zoom();
			let _ = api.vm_write(thirdperson_ptr, if is_ads { &-1i32 } else { &1i32 });
			if self.config.thirdperson_shoulder && ctx.data.player_third_person_shoulder_view != 0 {
				if let Some(local) = ctx.state.local_player() {
					if ctx.entity_check(api, local.index, local.entity_ptr) {
						let _ = api.vm_write(local.entity_ptr.field(ctx.data.player_third_person_shoulder_view), if is_ads { &0i32 } else { &1i32 });
					}
				}
			}
		}
		else if self.thirdperson {
			let _ = api.vm_write(thirdperson_ptr, &-1i32);
			if self.config.thirdperson_shoulder && ctx.data.player_third_person_shoulder_view != 0 {
				if let Some(local) = ctx.state.local_player() {
					if ctx.entity_check(api, local.index, local.entity_ptr) {
						let _ = api.vm_write(local.entity_ptr.field(ctx.data.player_third_person_shoulder_view), &0i32);
					}
				}
			}
		}
		self.thirdperson = self.config.thirdperson;
	}
	fn freecam(&mut self, api: &mut Api, ctx: &mut RunContext) {
		let Some(player) = ctx.state.local_player() else { return };

		// Track when freecam flag changes
		let changed = self.config.freecam != self.freecam;
		self.freecam = self.config.freecam;

		let mode = if self.freecam {
			if player.observer_mode == sdk::OBS_MODE_ROAMING {
				return;
			}
			sdk::OBS_MODE_ROAMING
		}
		else if changed {
			sdk::OBS_MODE_NONE
		}
		else {
			return;
		};
		if ctx.entity_check(api, player.index, player.entity_ptr) {
			let _ = api.vm_write(player.entity_ptr.field(ctx.data.player_observer_state), &mode);
		}
	}
	fn loot(&mut self, api: &mut Api, ctx: &mut RunContext) {
		if !self.config.fastloot {
			return;
		}
		ctx.inuse.force();

		let Some(player) = ctx.state.local_player() else { return };
		if !player.is_alive() {
			return;
		}

		let autoloot = ctx.state.is_button_down(self.config.autoloot);

		for loot in ctx.state.entities_as::<LootEntity>() {
			let filter = match loot.known_item {
				sdk::ItemId::Battery => autoloot,
				sdk::ItemId::ShieldCell => autoloot,
				sdk::ItemId::MedKit => autoloot,
				sdk::ItemId::PhoenixKit => autoloot,
				_ => false,
			};
			if (ctx.state.in_use() || filter) && sdk::dist2(player.view_origin, loot.origin) < 150.0 * 150.0 {
				let ptr = loot.entity_ptr.field(ctx.data.entity_highlight + 3 * 4 * 2 * sdk::HIGHLIGHT_MAX_CONTEXTS as u32);
				if let Ok(bits) = api.vm_read::<sdk::HighlightBits>(ptr) {
					if bits.outline_radius >= 128 {
						ctx.inuse = if ctx.rapidfire() { InState::Press } else { InState::Release };
					}
				}
			}
		}
	}
	fn superglide(&mut self, ctx: &mut RunContext) {
		if !self.config.superglide || ctx.state.time < 1.0 {
			return;
		}

		let Some(local) = ctx.state.local_player() else { return };

		if ctx.state.in_forward() {
			if local.velocity == [0.0; 3] {
				if self.climbtime == 0.0 {
					self.climbtime = ctx.state.time;
				}
			}
			else {
				if self.climbtime > 0.0 && ctx.state.time >= self.climbtime + 0.1 {
					if self.glidetime == 0.0 {
						self.glidetime = ctx.state.time;
					}
				}
				self.climbtime = 0.0;
			}
		}
		else {
			self.climbtime = 0.0;
		}

		ctx.jump.force();
		ctx.duck.force();

		if ctx.state.time >= self.glidetime && ctx.state.time < self.glidetime + 0.1 {
			ctx.jump = InState::Press;
			ctx.duck = InState::Press;
		}
		else {
			self.glidetime = 0.0;
		}
	}
}
