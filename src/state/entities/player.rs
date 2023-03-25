use super::*;

#[derive(Default)]
pub struct PlayerEntity {
	pub entity_ptr: sdk::Ptr,
	pub entity_size: u32,
	pub index: u32,

	pub origin: [f32; 3],
	pub angles: [f32; 3],
	pub velocity: [f32; 3],
	pub accel: [f32; 3],
	pub derivative_origin: EstimateDerivative,
	pub derivative_velocity: EstimateDerivative,

	pub trail: Vec<[f32; 3]>,
	pub trail_next: f64,

	pub health: i32,
	pub max_health: i32,
	pub shields: i32,
	pub max_shields: i32,
	pub ground_entity: sdk::EHandle,
	pub max_speed: f32,
	pub health_history: Vec<ValueChanged<i32>>,

	pub team_num: i32,
	pub team_color: [u8; 3],
	pub team_member_index: i32,
	pub squad_id: i32,
	pub grade: i32,

	pub model_name: ModelName,
	pub studio: StudioModel,
	pub bones: super::BoneArray,

	pub camera_origin: [f32; 3],
	pub camera_angles: [f32; 3],

	pub time_base: f32,
	pub server_angles: [f32; 3],

	pub consumables: [sdk::ConsumableItem; 16],

	pub view_offset: [f32; 3],
	pub view_origin: [f32; 3],
	pub view_angles: [f32; 3],
	pub breath_angles: [f32; 3],
	pub weapon_punch: [f32; 3],

	pub observer_mode: i32,
	pub observer_target: sdk::EHandle,

	pub flags: u32, //0x1: onground, 0x2: ducking
	pub life_state: u8,
	pub bleedout_state: i32,
	pub last_visible_time: f32,
	pub crosshair_target_start_time: f32,
	pub last_crosshair_target_time: f32,
	pub is_visible: bool,
	pub visible_time: f64,

	// Player WeaponInventory
	// Holds your weapons, grenade and character abilities
	pub weapons: [sdk::EHandle; 5],
	pub active_weapon: sdk::EHandle,
	pub active_weapon_time: f64,

	pub zooming: bool,

	pub helmet_type: i32,
	pub armor_type: i32,

	pub next_attack: f32,
	pub last_fired_time: f32,
	pub last_fired_weapon: sdk::EHandle,
	pub raise_from_melee_end_time: f32,

	// Weapon slots:
	// 0: primary weapon1
	// 1: primary weapon2
	// 2: melee
	// 4: grenade
	// 255: none
	pub selected_slot: u8,
	pub switchto_slot: u8,
	pub primary_weapon: sdk::EHandle,

	pub platform_uid: u64,
	pub eadp_uid: u64,
	pub cross_play: u8,
	pub cross_play_chat: u8,
	pub cross_play_chat_friends: u8,

	pub script_net_data_global: sdk::EHandle,
	pub script_net_data_exclusive: sdk::EHandle,
}
impl PlayerEntity {
	pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
		let entity_size = cc.ClassSize;
		Box::new(PlayerEntity { entity_ptr, entity_size, index, ..PlayerEntity::default() }) as Box<dyn Entity>
	}
	pub fn is_alive(&self) -> bool {
		self.life_state == 0
	}
	pub fn is_onground(&self) -> bool {
		self.flags & 0x1 != 0
	}
	pub fn is_ducking(&self) -> bool {
		self.flags & 0x2 != 0
	}
	pub fn is_downed(&self) -> bool {
		self.bleedout_state != 0
	}
	pub fn height(&self) -> f32 {
		if self.is_downed() { 25.0 }
		else if self.is_ducking() { 36.0 }
		else { 72.0 }
	}
	/// Returns the set of items which are of interest to this player.
	pub fn desired_items(&self, state: Option<&GameState>) -> sdk::ItemSet {
		use bitset_core::*;

		let mut set = bitset! {
			sdk::ItemSet::default();
			sdk::ItemId::UltAccel.0,
			sdk::ItemId::PhoenixKit.0,
			sdk::ItemId::MedKit.0,
			sdk::ItemId::Battery.0,
			sdk::ItemId::HelmetLv4.0,
			sdk::ItemId::BodyArmorLv3.0,
			sdk::ItemId::BodyArmorLv4.0,
			sdk::ItemId::EvoShieldLv3.0,
			sdk::ItemId::EvoShieldLv4.0,
			sdk::ItemId::KnockdownShieldLv3.0,
			sdk::ItemId::KnockdownShieldLv4.0,
			sdk::ItemId::BackpackLv2.0,
			sdk::ItemId::BackpackLv3.0,
			sdk::ItemId::BackpackLv4.0,
			sdk::ItemId::Thermite.0,
			sdk::ItemId::FragGrenade.0,
			sdk::ItemId::ArcStar.0,
			sdk::ItemId::LegendaryHopUp4.0,
			sdk::ItemId::HeavyMagazineLv4.0,
			sdk::ItemId::Keycard.0,
			sdk::ItemId::HeatShield.0,
			sdk::ItemId::MobileRespawn.0,
			sdk::ItemId::MrvnArm.0,
		};

		let needs_health = self.health < self.max_health;
		let needs_shields = self.shields < self.max_shields;

		// Healing items
		if needs_health {
			set.bit_set(sdk::ItemId::Syringe.0 as usize);
		}

		if let Some(state) = state {
			let syringes = self.get_consumable_count(state, sdk::ItemId::Syringe);
			let cells = self.get_consumable_count(state, sdk::ItemId::ShieldCell);
			if syringes < 4 || syringes % 4 != 0 {
				set.bit_set(sdk::ItemId::Syringe.0 as usize);
			}
			if cells < 8 || cells % 4 != 0 {
				set.bit_set(sdk::ItemId::ShieldCell.0 as usize);
			}
		}
		else {
			set.bit_set(sdk::ItemId::ShieldCell.0 as usize);
		}

		// Helmets
		if self.helmet_type < 3 {
			set.bit_set(sdk::ItemId::HelmetLv3.0 as usize);
			if self.helmet_type < 2 {
				set.bit_set(sdk::ItemId::HelmetLv2.0 as usize);
				if self.helmet_type < 1 {
					set.bit_set(sdk::ItemId::HelmetLv1.0 as usize);
				}
			}
		}

		// Body armors
		if self.armor_type < 1 || self.armor_type == 1 && needs_shields {
			set.bit_set(sdk::ItemId::BodyArmorLv1.0 as usize);
			set.bit_set(sdk::ItemId::EvoShieldLv1.0 as usize);
		}
		if self.armor_type < 2 || self.armor_type == 2 && needs_shields {
			set.bit_set(sdk::ItemId::BodyArmorLv2.0 as usize);
			set.bit_set(sdk::ItemId::EvoShieldLv2.0 as usize);
		}

		return set;
	}
	pub fn active_weapon<'a>(&self, state: &'a GameState) -> Option<&'a WeaponXEntity> {
		state.entity_as(self.active_weapon)
	}
	pub fn accel_angle(&self) -> f32 {
		let Some((_, olddir)) = self.derivative_velocity.history.first() else { return 0.0 };
		let olddir = [olddir[0], olddir[1], 0.0];
		let olddir = sdk::norm(olddir);
		if olddir == [0.0; 3] {
			return 0.0;
		}
		let veldir = [self.velocity[0], self.velocity[1], 0.0];
		let veldir = sdk::norm(veldir);
		if veldir == [0.0; 3] {
			return 0.0;
		}

		f32::acos(f32::clamp(sdk::dot(olddir, veldir), -1.0, 1.0)).to_degrees()
	}
	pub fn get_consumable_count(&self, state: &GameState, item: sdk::ItemId) -> i32 {
		let mut count = 0;
		if item != sdk::ItemId::None {
			for slot in &self.consumables {
				if state.known_item(slot.item as i32) == item {
					count += slot.count as i32;
				}
				if slot.count == 0 {
					break;
				}
			}
		}
		return count;
	}
}
impl Entity for PlayerEntity {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_ref(&self) -> EntityRef<'_> {
		EntityRef::Player(self)
	}
	fn is_serialized(&self) -> bool {
		true
	}
	fn get_info(&self) -> EntityInfo {
		EntityInfo {
			entity_ptr: self.entity_ptr,
			index: self.index as usize,
			handle: sdk::EHandle::from(self.index),
			rate: 1,
		}
	}
	fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
		#[derive(sdk::Pod)]
		#[repr(C)]
		struct Indices {
			origin: [u32; 6],
			velocity: [u32; 3],
			health: [u32; 6],
			team: [u32; 4],
			camera: [u32; 6],
			time_base: u32,
			server_angles: [u32; 3],
			breath_angles: [u32; 3],
			view_angles: [u32; 3],
			weapon_punch: [u32; 3],
			consumables: [u32; 16],
			view: [u32; 3],
			observer: [u32; 2],
			state: [u32; 6],
			inventory: [u32; 6],
			zoom_state: u32,
			armor_type: [u32; 2],
			next_attack: [u32; 4],
			selected: [u32; 3],
			uid: [u32; 4],
			model_name: [u32; 2],
			bone_array: [u32; 2],
			studio: [u32; 2],
			script: [u32; 2],
		}

		let data = ctx.data;
		let mut indices = Indices {
			origin: [
				data.entity_origin + 0,
				data.entity_origin + 4,
				data.entity_origin + 8,
				data.entity_origin + 24,
				data.entity_origin + 28,
				data.entity_origin + 32],
			velocity: [
				data.entity_velocity + 0,
				data.entity_velocity + 4,
				data.entity_velocity + 8],
			health: [
				data.entity_health,
				data.entity_max_health,
				data.entity_shield_health,
				data.entity_shield_health + 4,
				data.entity_health - 4,
				data.entity_health + 4],
			team: [
				data.entity_team_num,
				data.entity_team_num + 8,
				data.entity_team_num + 12,
				data.entity_team_num + 16],
			camera: [
				data.player_camera_data + 0,
				data.player_camera_data + 4,
				data.player_camera_data + 8,
				data.player_camera_data + 12,
				data.player_camera_data + 16,
				data.player_camera_data + 20],
			time_base:
				data.player_time_base,
			server_angles: [
				data.player_server_angles + 0,
				data.player_server_angles + 4,
				data.player_server_angles + 8,
			],
			breath_angles: [
				data.player_view_angles - 0x10 + 0,
				data.player_view_angles - 0x10 + 4,
				data.player_view_angles - 0x10 + 8,
			],
			view_angles: [
				data.player_view_angles + 0,
				data.player_view_angles + 4,
				data.player_view_angles + 8,
			],
			weapon_punch: [
				data.player_weapon_punch + 0,
				data.player_weapon_punch + 4,
				data.player_weapon_punch + 8,
			],
			consumables: [
				data.player_consumables + 0,
				data.player_consumables + 4,
				data.player_consumables + 8,
				data.player_consumables + 12,
				data.player_consumables + 16,
				data.player_consumables + 20,
				data.player_consumables + 24,
				data.player_consumables + 28,
				data.player_consumables + 32,
				data.player_consumables + 36,
				data.player_consumables + 40,
				data.player_consumables + 44,
				data.player_consumables + 48,
				data.player_consumables + 52,
				data.player_consumables + 56,
				data.player_consumables + 60,
			],
			view: [
				data.entity_view_offset + 0,
				data.entity_view_offset + 4,
				data.entity_view_offset + 8],
			observer: [
				data.player_observer_state + 0,
				data.player_observer_state + 4],
			state: [
				data.entity_flags,
				data.entity_life_state,
				data.player_bleedout_state,
				data.bcc_last_visible_time,
				data.bcc_last_visible_time + 4,
				data.bcc_last_visible_time + 8],
			inventory: [
				data.bcc_inventory + 8,
				data.bcc_inventory + 12,
				data.bcc_inventory + 16,
				data.bcc_inventory + 20,
				data.bcc_inventory + 24,
				data.bcc_inventory + 0x58],
			zoom_state:
				data.player_zoom_state,
			armor_type: [
				data.player_helmet_armor_type + 0,
				data.player_helmet_armor_type + 4],
			next_attack: [
				data.bcc_next_attack + 0,
				data.bcc_next_attack + 4,
				data.bcc_next_attack + 8,
				data.bcc_next_attack + 12],
			selected: [
				data.bcc_selected_weapons + 0,
				data.bcc_selected_weapons + 4,
				data.bcc_selected_weapons + 20],
			uid: [
				data.player_platform_uid + 0,
				data.player_platform_uid + 4,
				data.player_platform_uid + 16,
				data.player_platform_uid + 20],
			model_name: [
				data.entity_model_name + 0,
				data.entity_model_name + 4],
			bone_array: [
				data.animating_bone_array + 0,
				data.animating_bone_array + 4],
			studio: [
				data.animating_studiohdr + 0,
				data.animating_studiohdr + 4],
			script: [
				data.player_script_net_data + 0,
				data.player_script_net_data + 4],
		};

		if let Ok(fields) = api.vm_gatherd(self.entity_ptr, self.entity_size, &mut indices) {
			self.origin = [f32::from_bits(fields.origin[0]), f32::from_bits(fields.origin[1]), f32::from_bits(fields.origin[2])];
			self.angles = [f32::from_bits(fields.origin[3]), f32::from_bits(fields.origin[4]), f32::from_bits(fields.origin[5])];
			self.velocity = [f32::from_bits(fields.velocity[0]), f32::from_bits(fields.velocity[1]), f32::from_bits(fields.velocity[2])];

			let estvel = self.derivative_origin.update(ctx.time, self.origin, 0.1);
			if self.velocity == [0.0; 3] {
				self.velocity = estvel;
			}
			self.accel = self.derivative_velocity.update(ctx.time, self.velocity, 0.1);

			if ctx.time >= self.trail_next {
				if self.origin == [0.0; 3] {
					self.trail.clear();
				}
				else {
					if self.trail.len() >= 20 {
						self.trail.remove(0);
					}
					self.trail.push(self.origin);
				}
				self.trail_next = if self.trail_next + 0.2 > ctx.time { self.trail_next + 0.2 } else { ctx.time + 0.2 };
			}

			let health = fields.health[0] as i32;
			let shields = fields.health[2] as i32;

			if health + shields != self.health + self.shields {
				self.health_history.push(ValueChanged::new(ctx.time, self.health + self.shields, health + shields));
			}

			self.health = health;
			self.max_health = fields.health[1] as i32;
			self.shields = shields;
			self.max_shields = fields.health[3] as i32;
			self.ground_entity = sdk::EHandle::from(fields.health[4]);
			self.max_speed = f32::from_bits(fields.health[5]);

			self.team_num = fields.team[0] as i32;
			self.team_member_index = fields.team[1] as i32;
			self.squad_id = fields.team[2] as i32;
			self.grade = fields.team[3] as i32;
			// TODO: Figure out the team's trail color instead of making something up...
			let team_color = if self.team_num < 1 { [255, 255, 255] } else { TEAM_COLORS[(self.team_num - 1) as usize % TEAM_COLORS.len()] };
			self.team_color = team_color;

			self.camera_origin = [f32::from_bits(fields.camera[0]), f32::from_bits(fields.camera[1]), f32::from_bits(fields.camera[2])];
			self.camera_angles = [f32::from_bits(fields.camera[3]), f32::from_bits(fields.camera[4]), f32::from_bits(fields.camera[5])];

			self.time_base = f32::from_bits(fields.time_base);

			self.server_angles[0] = f32::from_bits(fields.server_angles[0]);
			self.server_angles[1] = f32::from_bits(fields.server_angles[1]);
			self.server_angles[2] = f32::from_bits(fields.server_angles[2]);

			self.breath_angles[0] = f32::from_bits(fields.breath_angles[0]);
			self.breath_angles[1] = f32::from_bits(fields.breath_angles[1]);
			self.breath_angles[2] = f32::from_bits(fields.breath_angles[2]);
			self.view_angles[0] = f32::from_bits(fields.view_angles[0]);
			self.view_angles[1] = f32::from_bits(fields.view_angles[1]);
			self.view_angles[2] = f32::from_bits(fields.view_angles[2]);
			self.weapon_punch[0] = f32::from_bits(fields.weapon_punch[0]);
			self.weapon_punch[1] = f32::from_bits(fields.weapon_punch[1]);
			self.weapon_punch[2] = f32::from_bits(fields.weapon_punch[2]);

			self.view_offset = [f32::from_bits(fields.view[0]), f32::from_bits(fields.view[1]), f32::from_bits(fields.view[2])];
			self.view_origin = sdk::add(self.origin, self.view_offset);

			dataview::bytes_mut(&mut self.consumables).copy_from_slice(dataview::bytes(&fields.consumables));

			self.observer_mode = fields.observer[0] as i32;
			self.observer_target = sdk::EHandle::from(fields.observer[1]);

			self.flags = fields.state[0];
			self.life_state = fields.state[1] as u8;
			self.bleedout_state = fields.state[2] as i32;
			self.last_visible_time = f32::from_bits(fields.state[3]);
			self.crosshair_target_start_time = f32::from_bits(fields.state[4]);
			self.last_crosshair_target_time = f32::from_bits(fields.state[5]);

			self.weapons[0] = sdk::EHandle::from(fields.inventory[0]);
			self.weapons[1] = sdk::EHandle::from(fields.inventory[1]);
			self.weapons[2] = sdk::EHandle::from(fields.inventory[2]);
			self.weapons[3] = sdk::EHandle::from(fields.inventory[3]);
			self.weapons[4] = sdk::EHandle::from(fields.inventory[4]);
			let active_weapon = sdk::EHandle::from(fields.inventory[5]);
			if self.active_weapon != active_weapon {
				self.active_weapon_time = ctx.time;
			}
			self.active_weapon = active_weapon;

			self.zooming = fields.zoom_state & 0x0000ff00 != 0;

			self.helmet_type = fields.armor_type[0] as i32;
			self.armor_type = fields.armor_type[1] as i32;

			self.next_attack = f32::from_bits(fields.next_attack[0]);
			self.last_fired_time = f32::from_bits(fields.next_attack[1]);
			self.last_fired_weapon = sdk::EHandle::from(fields.next_attack[2]);
			self.raise_from_melee_end_time = f32::from_bits(fields.next_attack[3]);

			self.switchto_slot = fields.selected[0].to_ne_bytes()[0];
			self.primary_weapon = fields.selected[1].into();
			self.selected_slot = fields.selected[2].to_ne_bytes()[0];

			self.platform_uid = fields.uid[0] as u64 | (fields.uid[1] as u64) << 32;
			self.eadp_uid = fields.uid[2] as u64 | (fields.uid[3] as u64) << 32;

			let model_name_ptr = fields.model_name[0] as u64 | (fields.model_name[1] as u64) << 32;
			self.model_name.update(api, model_name_ptr.into());
			let studio_ptr = fields.studio[0] as u64 | (fields.studio[1] as u64) << 32;
			self.studio.update(api, sdk::Ptr::from_raw(studio_ptr));
			let bone_ptr = fields.bone_array[0] as u64 | (fields.bone_array[1] as u64) << 32;
			self.bones.update(api, ctx, &self.studio, sdk::Ptr::from_raw(bone_ptr));

			self.script_net_data_global = sdk::EHandle::from(fields.script[0]);
			self.script_net_data_exclusive = sdk::EHandle::from(fields.script[1]);
		}
	}
	fn post(&mut self, _api: &mut Api, ctx: &UpdateContext, state: &GameState) {
		// Check if player is visible
		let is_visible = self.last_visible_time > 0.0 && (self.last_visible_time - state.client.curtime).abs() < 0.1;
		// Take note when the player became visible
		if !self.is_visible && is_visible {
			self.visible_time = ctx.time;
		}
		self.is_visible = is_visible;
	}
}

// Little hack to keep colors in sync everywhere
static TEAM_COLORS: [[u8; 3]; 21] = [
	[248, 104, 104],
	[242, 86, 38],
	[97, 92, 81],
	[174, 247, 89],
	[102, 214, 173],
	[98, 244, 234],
	[92, 208, 250],
	[93, 137, 238],
	[164, 105, 252],
	[243, 98, 161],
	[214, 67, 67],
	[230, 116, 51],
	[185, 179, 167],
	[148, 200, 65],
	[86, 174, 91],
	[55, 188, 200],
	[84, 169, 212],
	[98, 121, 203],
	[102, 61, 174],
	[218, 73, 145],
	[158, 178, 199],
];
