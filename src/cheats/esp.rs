use crate::base::solver::{LinearPredictor, solve, TargetPredictor};
use crate::*;
use super::espdata::{self, Flags, Object};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Fade {
	Never,
	Far,
	Normal,
	Near,
}
impl Default for Fade {
	fn default() -> Fade { Fade::Normal }
}

pub struct Config {
	pub enable: bool,
	pub team: bool,
	pub debug_ents: bool,
	pub debug_bones: bool,
	pub debug_loot: bool,
	pub debug_models: bool,
	pub debug_local: bool,
	pub aim_bone1: u32,
	pub aim_bone2: u32,
	pub distance: f32,
	pub trail_fade: f32,
	pub flags_player: Flags,
	pub flags_downed: Flags,
	pub flags_npc: Flags,
	pub flags_deathbox: Flags,
	pub flags_loot: Flags,
	pub flags_anim: Flags,
	pub flags_vehicle: Flags,
}
impl Default for Config {
	fn default() -> Config {
		Config {
			enable: true,
			team: false,
			aim_bone1: 11,
			aim_bone2: 5,
			debug_ents: false,
			debug_bones: false,
			debug_loot: false,
			debug_models: false,
			debug_local: false,
			distance: 18000.0,
			trail_fade: 30.0,
			flags_player: Flags::BOUNDS | Flags::HEALTH | Flags::AIM | Flags::SKYDOT,
			flags_downed: Flags::NAME | Flags::BOUNDS | Flags::FADED,
			flags_npc: Flags::TEXT | Flags::AIM | Flags::BONES | Flags::SKYDOT,
			flags_deathbox: Flags::TEXT,
			flags_loot: Flags::ICON | Flags::TEXT,
			flags_anim: Flags::TEXT,
			flags_vehicle: Flags::TEXT,
		}
	}
}

#[derive(Default)]
pub struct ESP {
	config: Config,
}
impl cvar::IVisit for ESP {
	fn visit(&mut self, f: &mut dyn FnMut(&mut dyn cvar::INode)) {
		let default = Config::default();
		f(&mut cvar::Property(s!("enable"), &mut self.config.enable, &default.enable));
		f(&mut cvar::Property(s!("team"), &mut self.config.team, &default.team));
		f(&mut cvar::Property(s!("debug.ents"), &mut self.config.debug_ents, &default.debug_ents));
		f(&mut cvar::Property(s!("debug.bones"), &mut self.config.debug_bones, &default.debug_bones));
		f(&mut cvar::Property(s!("debug.loot"), &mut self.config.debug_loot, &default.debug_loot));
		f(&mut cvar::Property(s!("debug.models"), &mut self.config.debug_models, &default.debug_models));
		f(&mut cvar::Property(s!("debug.local"), &mut self.config.debug_local, &default.debug_local));
		f(&mut cvar::Property(s!("distance"), &mut self.config.distance, &default.distance));
		f(&mut cvar::Property(s!("trail_fade"), &mut self.config.trail_fade, &default.trail_fade));
		f(&mut cvar::Property(s!("flags.player"), &mut self.config.flags_player, &default.flags_player));
		f(&mut cvar::Property(s!("flags.downed"), &mut self.config.flags_downed, &default.flags_downed));
		f(&mut cvar::Property(s!("flags.npc"), &mut self.config.flags_npc, &default.flags_npc));
		f(&mut cvar::Property(s!("flags.deathbox"), &mut self.config.flags_deathbox, &default.flags_deathbox));
		f(&mut cvar::Property(s!("flags.loot"), &mut self.config.flags_loot, &default.flags_loot));
		f(&mut cvar::Property(s!("flags.anim"), &mut self.config.flags_anim, &default.flags_anim));
		f(&mut cvar::Property(s!("flags.vehicle"), &mut self.config.flags_vehicle, &default.flags_vehicle));
	}
}
impl ESP {
	#[inline(never)]
	pub fn render(&mut self, api: &mut Api, ctx: &RunContext) {
		if !self.config.enable {
			return;
		}

		let Some(local) = ctx.state.local_player() else { return };

		// Precalculate the set of desired items
		// Only show desired items if no active weapon or if weapon is melee
		let desired_items =
			if ctx.state.player_is_melee(local) { ctx.state.desired_items(local) }
			else { sdk::ItemSet::default() };

		let ref mut objects = Vec::new();
		for entity in ctx.state.entities() {
			match entity.as_ref() {
				EntityRef::Player(player) => self.player(ctx, objects, local, player),
				EntityRef::BaseNPC(npc) => self.npc(ctx, objects, local, npc),
				EntityRef::Deathbox(deathbox) => self.deathbox(ctx, objects, local, deathbox),
				EntityRef::Loot(loot) => self.loot(ctx, objects, local, loot, &desired_items),
				EntityRef::Animating(anim) => self.animating(ctx, objects, local, anim),
				EntityRef::Vehicle(vehicle) => self.vehicle(ctx, objects, local, vehicle),
				_ => (),
			}
		}

		// Sort back-to-front
		// Object struct is large, sort references instead
		let mut objects: Vec<&Object> = objects.iter().collect();
		objects.sort_unstable_by(|&lhs, &rhs| f32::total_cmp(&rhs.distance, &lhs.distance));

		let ref conf = espdata::Config {
			debug_bones: self.config.debug_bones,
			debug_models: self.config.debug_models,
			fade_distance: self.config.distance,
			fade_factor: 10.0,
			icon_grid: 47.0,
			icon_image: 0x76b36395,
			bounds_scale: 1.5,
			bounds_trans: 5.0,
		};
		for object in objects {
			object.draw(api, ctx, conf);
		}
	}
}

/// Transform entities into ESP objects.
impl ESP {
	fn player<'a>(&self, ctx: &RunContext<'a>, objects: &mut Vec<Object<'a>>, local: &'a PlayerEntity, player: &'a PlayerEntity) {
		if !player.is_alive() {
			return;
		}
		if ctx.state.is_same_team(local, player) {
			if local.index == player.index {
				if !self.config.debug_local {
					return;
				}
			}
			else if !self.config.team {
				return;
			}
		}

		let (skynade_pitch, skynade_yaw) = skynade_angle(ctx.state, local, &player.origin);

		let va = if player.index == local.index { &local.view_angles } else { &player.angles };
		let target = LinearPredictor {
			origin: sdk::add(player.origin, player.bones.get_pos(self.config.aim_bone1 as usize)),
			velocity: player.velocity,
		};

		objects.push(Object {
			name: ctx.state.get_player_name(player.get_info().handle),
			color: player.team_color.into(),
			visible: player.is_visible,
			fade_dist: self.get_fade(ctx.state, local, Fade::Normal),
			flags: if player.is_downed() { self.config.flags_downed } else { self.config.flags_player },
			origin: player.origin,
			distance: sdk::dist(player.origin, local.origin),
			bones: Some(&player.bones.v),
			studio: Some(&player.studio),
			spine: [player.bones.get_pos(self.config.aim_bone1 as usize), player.bones.get_pos(self.config.aim_bone2 as usize)],
			aim: self.aim(ctx.state, local, &target),
			skynade_pitch, skynade_yaw,
			view: sdk::qvec(*va),
			trail: Some(&player.trail),
			width: 36.0,
			height: player.height(),
			health: player.health,
			max_health: player.max_health,
			shields: player.shields,
			max_shields: player.max_shields,
			model_name: Some(&player.model_name.string),
			..Object::default()
		});
	}

	fn npc<'a>(&self, ctx: &RunContext<'a>, objects: &mut Vec<Object<'a>>, local: &'a PlayerEntity, npc: &'a BaseNPCEntity) {
		let is_melee = ctx.state.player_is_melee(local);
		let (class, fade) = match npc.model_name.hash {
			sdk::ModelName::LOOT_TICK if is_melee => if npc.skin == 0 {
				(strpool!(ctx, "tick"), Fade::Normal)
			} else {
				(strpool!(ctx, "TICK"), Fade::Far)
			},
			sdk::ModelName::MARVIN_BASE if is_melee => if npc.skin == 0 {
				(strpool!(ctx, "mrvn"), Fade::Normal)
			} else {
				(strpool!(ctx, "MRVN"), Fade::Far)
			},
			sdk::ModelName::PROWLER => (strpool!(ctx, "Prowler"), Fade::Normal),
			sdk::ModelName::DUMMIE_GENERIC | sdk::ModelName::DUMMIE_COMBAT | sdk::ModelName::DUMMIE_TRAINING => (strpool!(ctx, "DUMMIE"), Fade::Normal),
			sdk::ModelName::DUMMY => (strpool!(ctx, "DUMMY"), Fade::Normal),
			_ if !self.config.debug_models => return,
			_ => (strpool!(ctx, "BaseNPC"), Fade::Near),
		};

		let target = LinearPredictor {
			origin: sdk::add(npc.origin, npc.bones.get_pos(self.config.aim_bone1 as usize)),
			velocity: [0.0; 3],
		};

		let (skynade_pitch, skynade_yaw) = if ctx.state.is_firing_range() { skynade_angle(ctx.state, local, &npc.origin) } else { (0.0, 0.0) };

		objects.push(Object {
			text: Some(class),
			color: vgc::sRGB::White,
			visible: npc.is_visible,
			fade_dist: self.get_fade(ctx.state, local, fade),
			flags: self.config.flags_npc,
			origin: npc.origin,
			width: 36.0,
			height: 72.0,
			distance: sdk::dist(local.origin, npc.origin),
			bones: Some(&npc.bones.v),
			studio: Some(&npc.studio),
			spine: [npc.bones.get_pos(self.config.aim_bone1 as usize), npc.bones.get_pos(self.config.aim_bone2 as usize)],
			aim: self.aim(ctx.state, local, &target),
			skynade_pitch, skynade_yaw,
			model_name: Some(&npc.model_name.string),
			skin: npc.skin,
			..Object::default()
		});
	}

	fn deathbox<'a>(&self, ctx: &RunContext<'a>, objects: &mut Vec<Object<'a>>, local: &'a PlayerEntity, deathbox: &'a DeathboxEntity) {
		// Only show deathboxes while holding out melee
		if !ctx.state.player_is_melee(local) {
			return;
		}

		objects.push(Object {
			text: Some(strpool!(ctx, "LOOT")),
			color: vgc::sRGB::White,
			fade_dist: self.get_fade(ctx.state, local, Fade::Near),
			flags: self.config.flags_deathbox,
			origin: deathbox.origin,
			distance: sdk::dist(local.origin, deathbox.origin),
			..Object::default()
		});
	}

	fn loot<'a>(&self, ctx: &RunContext<'a>, objects: &mut Vec<Object<'a>>, local: &'a PlayerEntity, loot: &'a LootEntity, desired_items: &sdk::ItemSet) {
		// Loot in deathboxes show up at the world origin
		if loot.origin == [0.0; 3] {
			return;
		}

		let distance = sdk::dist(local.origin, loot.origin);

		// Special rules for ground loot weapons
		if !self.config.debug_loot && loot.weapon_name_index != 255 {
			// If the player doesn't have 2 weapons, add ESP for ground loot weapons
			if !(local.weapons[0].is_valid() && local.weapons[1].is_valid()) || ctx.state.player_is_melee(local) {
				objects.push(Object {
					text: match loot.weapon_name {
						sdk::WeaponName::MASTIFF => Some(strpool!(ctx, "Mastiff")),
						sdk::WeaponName::PEACEKEEPER => Some(strpool!(ctx, "PK")),
						sdk::WeaponName::R301 => Some(strpool!(ctx, "R301")),
						sdk::WeaponName::SENTINEL => Some(strpool!(ctx, "Sentinel")),
						sdk::WeaponName::FLATLINE => Some(strpool!(ctx, "Flatline")),
						sdk::WeaponName::WINGMAN => Some(strpool!(ctx, "Wingman")),
						sdk::WeaponName::PROWLER => Some(strpool!(ctx, "Prowler")),
						sdk::WeaponName::KRABER => Some(strpool!(ctx, "Kraber")),
						sdk::WeaponName::G7_SCOUT => Some(strpool!(ctx, "Scout")),
						sdk::WeaponName::BOCEK => Some(strpool!(ctx, "Bocek")),
						// sdk::WeaponName::RAMPAGE => Some(strpool!(ctx, "Rampage")),
						sdk::WeaponName::RE45 => Some(strpool!(ctx, "RE45")),
						sdk::WeaponName::NEMESIS => Some(strpool!(ctx, "Nemesis")),
						sdk::WeaponName::HEMLOK => Some(strpool!(ctx, "Hemlok")),
						sdk::WeaponName::EVA8_AUTO => Some(strpool!(ctx, "EVA8")),
						sdk::WeaponName::CAR => Some(strpool!(ctx, "C.A.R")),
						sdk::WeaponName::R99 => Some(strpool!(ctx, "R99")),
						sdk::WeaponName::HAVOC => Some(strpool!(ctx, "Havoc")),
						sdk::WeaponName::VOLT => Some(strpool!(ctx, "Volt")),
						_ => None,
					},
					color: vgc::sRGB::White,
					fade_dist: self.get_fade(ctx.state, local, Fade::Never),
					flags: Flags::TEXT,
					origin: loot.origin,
					distance,
					..Object::default()
				});
			}
			return;
		}

		// Debug missing loot
		if self.config.debug_loot || matches!(loot.known_item, sdk::ItemId::None) {
			let debug_print = fmtools::format!(
				"script int: "{loot.custom_script_int}"\n"
				"model name: "{loot.model_name.string}"\n"
				"bits: "{loot.bits.to_int():#010x}"\n"
				{loot.skin}" "{loot.skin_mod}" "{loot.body}" "{loot.camo_index});
			let debug_print = ctx.pool.store(debug_print);
			objects.push(Object {
				text: Some(debug_print),
				color: vgc::sRGB::White,
				fade_dist: self.get_fade(ctx.state, local, Fade::Never),
				flags: Flags::TEXT,
				origin: loot.origin,
				distance,
				model_name: Some(&loot.model_name.string),
				skin: loot.skin,
				..Object::default()
			});
			return;
		}

		// Draw loot icon
		let index = loot.known_item.0 as usize;
		if index < desired_items.bit_len() && desired_items.bit_test(index) {
			objects.push(Object {
				color: vgc::sRGB::White,
				fade_dist: self.get_fade(ctx.state, local, Fade::Never),
				flags: self.config.flags_loot,
				origin: loot.origin,
				distance,
				icon: super::Icon::find(loot.known_item).map(|icon| espdata::Icon { x: icon.gridx, y: icon.gridy }),
				..Object::default()
			});
		}
	}

	fn animating<'a>(&self, ctx: &RunContext<'a>, objects: &mut Vec<Object<'a>>, local: &'a PlayerEntity, anim: &'a AnimatingEntity) {
		let (class, fade) = match anim.model_name.hash {
			sdk::ModelName::LOOT_SPHERE => (strpool!(ctx, "OO"), Fade::Normal),
			sdk::ModelName::LOOT_BIN if ctx.state.player_is_melee(local) => if anim.skin == 0 {
				(strpool!(ctx, "bin"), Fade::Near)
			} else {
				(strpool!(ctx, "BIN"), Fade::Near)
			},
			sdk::ModelName::GAS_TANK => (strpool!(ctx, "GAS"), Fade::Normal),
			sdk::ModelName::JUMP_PAD => (strpool!(ctx, "PAD"), Fade::Normal),
			sdk::ModelName::TOTEM => (strpool!(ctx, "TOTEM"), Fade::Normal),
			sdk::ModelName::PARIAH_DRONE => (strpool!(ctx, "SEER"), Fade::Normal),
			sdk::ModelName::TROPHY_SYSTEM => (strpool!(ctx, "PYLON"), Fade::Normal),
			// Draw a count down timer for the gibraltar dome shield
			sdk::ModelName::BUBBLESHIELD => {
				const DOME_TIME: f64 = 12.0 + 0.5; // 12s + 0.5s activation
				let timer = DOME_TIME - (ctx.state.time - anim.spawn_time);
				if timer < 0.0 {
					return;
				}
				(ctx.pool.store(fmtools::format!({timer:.1}" s")), Fade::Normal)
			},
			sdk::ModelName::EMPTY_MODEL => return,
			_ if !self.config.debug_models => return,
			_ => (strpool!(ctx, "Anim"), Fade::Near),
		};

		objects.push(Object {
			text: Some(class),
			color: vgc::sRGB::White,
			fade_dist: self.get_fade(ctx.state, local, fade),
			flags: self.config.flags_anim,
			origin: anim.origin,
			distance: sdk::dist(local.origin, anim.origin),
			model_name: Some(&anim.model_name.string),
			skin: anim.skin,
			..Object::default()
		});
	}

	fn vehicle<'a>(&self, ctx: &RunContext<'a>, objects: &mut Vec<Object<'a>>, local: &'a PlayerEntity, vehicle: &'a VehicleEntity) {
		objects.push(Object {
			text: Some(strpool!(ctx, "^.^")),
			color: ctx.state.entity_as::<PlayerEntity>(vehicle.vehicle_driver).map(|driver| driver.team_color.into()).unwrap_or(vgc::sRGB::White),
			fade_dist: self.get_fade(ctx.state, local, Fade::Normal),
			flags: self.config.flags_vehicle,
			origin: vehicle.origin,
			distance: sdk::dist(local.origin, vehicle.origin),
			..Object::default()
		});
	}

	fn aim(&self, state: &GameState, local: &PlayerEntity, target: &dyn TargetPredictor) -> Option<[f32; 3]> {
		let weapon = local.active_weapon(state)?;

		if weapon.projectile_speed <= 1.0 {
			return None;
		}

		let sol = solve(&local.view_origin, weapon, target)?;

		Some([-sol.pitch.to_degrees(), sol.yaw.to_degrees(), 0.0])
	}

	fn get_fade(&self, state: &GameState, local: &PlayerEntity, fade: Fade) -> f32 {
		// Factor in the zoom level to increase its effective range
		use std::f32::consts::TAU;
		let zoom = f32::tan(TAU / 8.0) / f32::tan(state.get_fov(local).to_radians() / 2.0);

		// If we're skydiving or in the dropship draw everything at all ranges
		// This gives us a good overview of the whole map
		let max_dist = match fade {
			Fade::Near => self.config.distance * 0.25,
			_ if false || local.camera_origin[2] > sdk::AIRSHIP_HEIGHT => return 1000000.0,
			Fade::Normal => self.config.distance,
			Fade::Far => self.config.distance * 8.0,
			Fade::Never => return 1000000.0,
		};

		return max_dist * zoom;
	}
}

fn skynade_angle(state: &GameState, local: &PlayerEntity, target: &[f32; 3]) -> (f32, f32) {
	let Some(weapon) = local.active_weapon(state) else { return Default::default() };

	let (lob, pitches, z_offset): (bool, &[sdk::pitches::Pitch], f32) =
	match (weapon.mod_bitfield & 0x4 != 0, weapon.weapon_name) {
		(false, sdk::WeaponName::THERMITE_GRENADE) => (false, &sdk::pitches::GRENADE_PITCHES, 0.0),
		(false, sdk::WeaponName::FRAG_GRENADE) => (true, &sdk::pitches::GRENADE_PITCHES, 70.0),
		(false, sdk::WeaponName::ARC_STAR) => (false, &sdk::pitches::ARC_PITCHES, 25.0),
		(true, sdk::WeaponName::THERMITE_GRENADE) => (false, &sdk::pitches::GRENADIER_GRENADE_PITCHES, 0.0),
		(true, sdk::WeaponName::FRAG_GRENADE) => (false, &sdk::pitches::GRENADIER_GRENADE_PITCHES, 70.0),
		(true, sdk::WeaponName::ARC_STAR) => (false, &sdk::pitches::GRENADIER_ARC_PITCHES, 25.0),
		_ => return Default::default(),
	};

	let g = 750.0 * weapon.projectile_scale;
	let v0 = weapon.projectile_speed;

	let delta = sdk::sub(*target, local.view_origin);
	let delta = sdk::add(delta, sdk::muls(delta, 20.0 / sdk::len(delta)));
	let dx = f32::sqrt(delta[0] * delta[0] + delta[1] * delta[1]);
	let dy = delta[2] + z_offset;

	let calc_angle = if lob { lob_angle } else { optimal_angle };
	if let Some(launch_pitch) = calc_angle(dx, dy, v0, g) {
		let view_pitch = sdk::pitches::launch2view(pitches, launch_pitch);
		return (view_pitch, sdk::qangle(sdk::sub(*target, local.view_origin))[1].to_radians());
	}
	else {
		return Default::default();
	}

	fn optimal_angle(x: f32, y: f32, v0: f32, g: f32) -> Option<f32> {
		let root = v0 * v0 * v0 * v0 - g * (g * x * x + 2.0 * y * v0 * v0);
		if root < 0.0 {
			return None;
		}
		let root = f32::sqrt(root);
		let slope = (v0 * v0 - root) / (g * x);
		Some(f32::atan(slope))
	}
	fn lob_angle(x: f32, y: f32, v0: f32, g: f32) -> Option<f32> {
		let root = v0 * v0 * v0 * v0 - g * (g * x * x + 2.0 * y * v0 * v0);
		if root < 0.0 {
			return None;
		}
		let root = f32::sqrt(root);
		let slope = (v0 * v0 + root) / (g * x);
		Some(f32::atan(slope))
	}
}
