use super::*;

#[derive(Default)]
pub struct AnimatingEntity {
	pub entity_ptr: sdk::Ptr,
	pub entity_size: u32,
	pub index: u32,
	pub origin: [f32; 3],

	update_rate: u32,
	update_time: f64,

	pub model_name: ModelName,

	pub owner_entity: sdk::EHandle,

	pub skin: i32,
	pub skin_mod: i32,
	pub body: i32,
	pub camo_index: i32,

	pub spawn_time: f64,
}
impl AnimatingEntity {
	pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
		let entity_size = cc.ClassSize;
		Box::new(AnimatingEntity { entity_ptr, entity_size, index, ..AnimatingEntity::default() })
	}
}
impl Entity for AnimatingEntity {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_ref(&self) -> EntityRef<'_> {
		EntityRef::Animating(self)
	}
	fn is_serialized(&self) -> bool {
		false
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
		if self.spawn_time == 0.0 {
			self.spawn_time = ctx.time;
		}

		#[derive(sdk::Pod)]
		#[repr(C)]
		struct Indices {
			origin: [u32; 3],
			model_name: [u32; 2],
			owner_entity: u32,
			skin: [u32; 4],
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
			owner_entity:
				data.entity_owner_entity,
			skin: [
				data.animating_skin + 0,
				data.animating_skin + 4,
				data.animating_skin + 8,
				data.animating_skin + 12],
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

			self.owner_entity = sdk::EHandle::from(fields.owner_entity);

			self.skin = fields.skin[0] as i32;
			self.skin_mod = fields.skin[1] as i32;
			self.body = fields.skin[2] as i32;
			self.camo_index = fields.skin[3] as i32;
		}

		self.update_rate = if ctx.time >= self.update_time + 0.25 { 512 } else { 4 };
	}
}
impl serde::Serialize for AnimatingEntity {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::*;
		let state = serializer.serialize_struct(unsafe_obfstr!("Animating"), 0)?;
		state.end()
	}
}
