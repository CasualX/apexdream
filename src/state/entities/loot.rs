use super::*;

#[derive(Default)]
pub struct LootEntity {
	pub entity_ptr: sdk::Ptr,
	pub entity_size: u32,
	pub index: u32,
	pub origin: [f32; 3],

	update_rate: u32,
	update_time: f64,

	pub model_name: ModelName,

	pub skin: i32,
	pub skin_mod: i32,
	pub body: i32,
	pub camo_index: i32,

	pub color: [f32; 3],
	pub bits: sdk::HighlightBits,

	pub ammo_in_clip: i32,
	pub custom_script_int: i32,
	pub known_item: sdk::ItemId,
	pub survival_property: i32,
	pub weapon_name_index: i32,
	pub weapon_name: sdk::WeaponName,
	pub mod_bitfield: u32,
}
impl LootEntity {
	pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
		let entity_size = cc.ClassSize;
		Box::new(LootEntity { entity_ptr, entity_size, index, ..LootEntity::default() }) as Box<dyn Entity>
	}
}
impl Entity for LootEntity {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_ref(&self) -> EntityRef<'_> {
		EntityRef::Loot(self)
	}
	fn is_serialized(&self) -> bool {
		true
	}
	fn get_info(&self) -> EntityInfo {
		EntityInfo {
			entity_ptr: self.entity_ptr,
			index: self.index as usize,
			handle: sdk::EHandle::from(self.index),
			rate: self.update_rate,
		}
	}
	fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
		#[derive(sdk::Pod)]
		#[repr(C)]
		struct Indices {
			origin: [u32; 3],
			model_name: [u32; 2],
			skin: [u32; 4],
			highlight: [u32; 4],
			survival: [u32; 5]
		}

		let data = ctx.data;
		let mut indices = Indices {
			origin: [
				data.entity_origin + 0,
				data.entity_origin + 4,
				data.entity_origin + 8],
			model_name: [
				data.entity_model_name + 0,
				data.entity_model_name + 4],
			skin: [
				data.animating_skin + 0,
				data.animating_skin + 4,
				data.animating_skin + 8,
				data.animating_skin + 12],
			highlight: [
				data.entity_highlight + 0,
				data.entity_highlight + 4,
				data.entity_highlight + 8,
				data.entity_highlight + 4 * 3 * 2 * sdk::HIGHLIGHT_MAX_CONTEXTS as u32],
			survival: [
				data.prop_survival + 0,
				data.prop_survival + 4,
				data.prop_survival + 8,
				data.prop_survival + 16,
				data.prop_survival + 20],
		};

		if let Ok(fields) = api.vm_gatherd(self.entity_ptr, self.entity_size, &mut indices) {
			let origin = [
				f32::from_bits(fields.origin[0]),
				f32::from_bits(fields.origin[1]),
				f32::from_bits(fields.origin[2])];
			if self.origin != origin {
				self.update_time = ctx.time;
			}
			self.origin = origin;

			let model_name_ptr = fields.model_name[0] as u64 | (fields.model_name[1] as u64) << 32;
			self.model_name.update(api, model_name_ptr.into());

			self.skin = fields.skin[0] as i32;
			self.skin_mod = fields.skin[1] as i32;
			self.body = fields.skin[2] as i32;
			self.camo_index = fields.skin[3] as i32;

			self.color[0] = f32::from_bits(fields.highlight[0]);
			self.color[1] = f32::from_bits(fields.highlight[1]);
			self.color[2] = f32::from_bits(fields.highlight[2]);
			self.bits = sdk::HighlightBits::from_uint(fields.highlight[3]);

			self.ammo_in_clip = fields.survival[0] as i32;
			self.custom_script_int = fields.survival[1] as i32;
			self.survival_property = fields.survival[2] as i32;
			self.weapon_name_index = fields.survival[3] as u16 as i32;
			self.mod_bitfield = fields.survival[4];
		}

		self.update_rate = if ctx.time >= self.update_time + 0.25 { 64 } else { 2 };
	}
	fn post(&mut self, _api: &mut Api, _ctx: &UpdateContext, state: &GameState) {
		self.weapon_name = state.weapon_name(self.weapon_name_index);
		self.known_item = state.known_item(self.custom_script_int);
	}
}
impl serde::Serialize for LootEntity {
	#[inline(never)]
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::*;
		let mut state = serializer.serialize_struct(unsafe_obfstr!("PropSurvival"), 5)?;
		state.serialize_field(unsafe_obfstr!("entity_ptr"), &self.entity_ptr)?;
		state.serialize_field(unsafe_obfstr!("index"), &self.index)?;
		state.serialize_field(unsafe_obfstr!("class_name"), s!("PropSurvival"))?;
		state.serialize_field(unsafe_obfstr!("origin"), &self.origin)?;
		state.serialize_field(unsafe_obfstr!("custom_script_int"), &self.known_item.0)?;
		state.end()
	}
}
