use crate::*;

mod entity_list;
mod input_system;
mod client_state;
mod buttons;
mod studio;
mod string_tables;
mod name_list;
mod script_data;
mod itemids;
mod modifiers;
mod derivative;
pub mod entities;

pub use self::studio::StudioModel;
pub use self::script_data::ScriptValue;
pub use self::derivative::EstimateDerivative;
pub use self::buttons::Buttons;

#[derive(Default)]
pub struct GameState {
	pub time: f64,
	pub client: client_state::ClientState,
	pub entity_list: entity_list::EntityList,
	pub input_system: input_system::InputSystem,
	pub string_tables: string_tables::StringTables,
	pub name_list: name_list::NameList,
	pub buttons: buttons::Buttons,
	pub script_data: script_data::ScriptNetData,
	pub items: itemids::LootItems,
	pub mods: modifiers::Modifiers,

	gamemode_buf: [u8; 16],
	gamemode_hash: u32,
}



impl GameState {
	#[inline(never)]
	pub fn update(&mut self, api: &mut Api, ctx: &mut UpdateContext) {
		self.client.update(api, ctx);
		self.entity_list.update(api, ctx);
		self.input_system.update(api, ctx);
		self.string_tables.update(api, ctx);
		self.name_list.update(api, ctx);
		self.buttons.update(api, ctx);
		self.script_data.update(api, ctx);
		self.items.update(api, ctx);
		self.mods.update(api, ctx);

		for i in 0..self.entity_list.entities.len() {
			// Temporarily take the entity out of the list
			if let Some(mut entity) = self.entity_list.entities.get_mut(i).and_then(|entity| entity.take()) {
				// Analyze the entities for derived information
				{
					let entity_ref = (&*entity).as_ref();
					self.items.visit(api, ctx, entity_ref);
					self.mods.visit(api, ctx, entity_ref);
				}
				// Update the entities with general game state information
				entity.post(api, ctx, self);
				// Place the entity back in the list
				if let Some(place @ &mut None) = self.entity_list.entities.get_mut(i) {
					*place = Some(entity);
				}
			}
		}

		if ctx.connected && ctx.data.mp_gamemode != 0 {
			self.gamemode_hash = 0;
			if let Ok(gamemode_ptr) = api.vm_read::<sdk::Ptr<[u8]>>(ctx.process.base.field(ctx.data.mp_gamemode + 0x58)) {
				if !gamemode_ptr.is_null() {
					if let Ok(gamemode) = api.vm_read_cstr(gamemode_ptr, &mut self.gamemode_buf) {
						self.gamemode_hash = crate::hash(gamemode);
					}
				}
			}
		}
	}
	pub fn gamemode(&self) -> Option<&str> {
		base::from_utf8_buf(&self.gamemode_buf)
	}
	#[inline(never)]
	pub fn get_config_section(&self) -> u32 {
		let Some(local) = self.local_player() else { return 0 };
		let Some(weapon) = local.active_weapon(self) else { return 0 };

		if self.weapon_is_melee(weapon.weapon_name_index) {
			return hash!("MELEE");
		}

		use sdk::WeaponName as WN;
		match weapon.weapon_name {
			WN::R301         => hash!("R301"),
			WN::SENTINEL     => hash!("SENTINEL"),
			WN::BOCEK        => hash!("BOCEK"),
			WN::ALTERNATOR   => hash!("ALTERNATOR"),
			WN::RE45         => hash!("RE45"),
			WN::CHARGE_RIFLE => hash!("CHARGE_RIFLE"),
			WN::DEVOTION     => hash!("DEVOTION"),
			WN::LONGBOW      => hash!("LONGBOW"),
			WN::HAVOC        => hash!("HAVOC"),
			WN::EVA8_AUTO    => hash!("EVA8_AUTO"),
			WN::FLATLINE     => hash!("FLATLINE"),
			WN::G7_SCOUT     => hash!("G7_SCOUT"),
			WN::HEMLOK       => hash!("HEMLOK"),
			WN::KRABER       => hash!("KRABER"),
			WN::LSTAR        => hash!("LSTAR"),
			WN::MASTIFF      => hash!("MASTIFF"),
			WN::MOZAMBIQUE   => hash!("MOZAMBIQUE"),
			WN::PROWLER      => hash!("PROWLER"),
			WN::PEACEKEEPER  => hash!("PEACEKEEPER"),
			WN::R99          => hash!("R99"),
			WN::P2020        => hash!("P2020"),
			WN::SPITFIRE     => hash!("SPITFIRE"),
			WN::TRIPLE_TAKE  => hash!("TRIPLE_TAKE"),
			WN::WINGMAN      => hash!("WINGMAN"),
			WN::VOLT         => hash!("VOLT"),
			WN::REPEATER     => hash!("REPEATER"),
			WN::CAR          => hash!("CAR"),
			WN::NEMESIS      => hash!("NEMESIS"),
			WN::THROWING_KNIFE => hash!("THROWING_KNIFE"),
			WN::VANTAGE_SNIPER => hash!("VANTAGE_SNIPER"),
			_ => hash!("ANY"),
		}
	}
	/// Returns if the players are on the same team.
	/// This matters in control mode where multiple squads can be in the same team.
	pub fn is_same_team(&self, p1: &PlayerEntity, p2: &PlayerEntity) -> bool {
		if self.gamemode_hash == sdk::GAMEMODE_CONTROL || self.gamemode_hash == sdk::GAMEMODE_FREEDM {
			p1.team_num & 1 == p2.team_num & 1
		}
		else {
			p1.team_num == p2.team_num
		}
	}
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub struct ValueChanged<T> {
	pub time: f64,
	pub old: T,
	pub new: T,
}
impl<T> ValueChanged<T> {
	pub const fn new(time: f64, old: T, new: T) -> ValueChanged<T> {
		ValueChanged { time, old, new }
	}
}

//----------------------------------------------------------------

pub struct UpdateContext<'a> {
	pub process: &'a GameProcess,
	pub data: &'a GameData,
	pub time: f64,
	pub tickcount: u32,

	// Connection state changed to fully connected
	pub connected: bool,
	// Prioritize updating local player related information
	pub local_entity: sdk::EHandle,
	// Update full bones instead of only spine
	pub full_bones: bool,
}

impl<'a> UpdateContext<'a> {
	/// Rate limit reading less important variables.
	#[inline]
	pub fn ticked(&self, rate: u32, offset: u32) -> bool {
		if rate <= 1 {
			return true;
		}
		(self.tickcount.wrapping_add(offset)) % rate == 0
	}
}

//----------------------------------------------------------------

impl GameState {
	pub fn get_fov(&self, player: &PlayerEntity) -> f32 {
		if player.zooming {
			if let Some(weapon) = self.entity_as::<WeaponXEntity>(player.active_weapon) {
				if weapon.target_zoom_fov > 1.0 && weapon.target_zoom_fov <= 90.0 {
					return weapon.target_zoom_fov;
				}
			}
		}
		return 90.0;
	}
	pub fn desired_items(&self, player: &PlayerEntity) -> sdk::ItemSet {
		// Start by collecting desired items from the player
		let mut desired_items = player.desired_items(Some(self));

		// Add the set of desired items from their primary and secondary weapon
		if let Some(weapon) = self.entity_as::<WeaponXEntity>(player.weapons[0]) {
			desired_items.bit_or(&weapon.desired_items(self));
		}
		if let Some(weapon) = self.entity_as::<WeaponXEntity>(player.weapons[1]) {
			desired_items.bit_or(&weapon.desired_items(self));
		}
		return desired_items;
	}
	pub fn player_is_melee(&self, player: &PlayerEntity) -> bool {
		if let Some(weapon) = player.active_weapon(self) {
			self.weapon_is_melee(weapon.weapon_name_index) || weapon.weapon_name == sdk::WeaponName::CONSUMABLE
		}
		else {
			true
		}
	}
}
