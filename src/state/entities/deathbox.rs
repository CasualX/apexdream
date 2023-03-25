use super::*;

#[derive(Default)]
pub struct DeathboxEntity {
	pub entity_ptr: sdk::Ptr,
	pub entity_size: u32,
	pub index: u32,
	pub origin: [f32; 3],

	pub color: [f32; 3],
}
impl DeathboxEntity {
	pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
		let entity_size = cc.ClassSize;
		Box::new(DeathboxEntity { entity_ptr, entity_size, index, ..DeathboxEntity::default() })
	}
}
impl Entity for DeathboxEntity {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_ref(&self) -> EntityRef<'_> {
		EntityRef::Deathbox(self)
	}
	fn is_serialized(&self) -> bool {
		false
	}
	fn get_info(&self) -> EntityInfo {
		EntityInfo {
			entity_ptr: self.entity_ptr,
			index: self.index as usize,
			handle: sdk::EHandle::from(self.index),
			rate: 128,
		}
	}
	fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
		#[derive(sdk::Pod)]
		#[repr(C)]
		struct Indices {
			origin: [u32; 3],
			highlight: [u32; 3],
		}

		let data = ctx.data;
		let mut indices = Indices {
			origin: [
				data.entity_origin + 0,
				data.entity_origin + 4,
				data.entity_origin + 8],
			highlight: [
				data.entity_highlight + 0,
				data.entity_highlight + 4,
				data.entity_highlight + 8],
		};

		if let Ok(fields) = api.vm_gatherd(self.entity_ptr, self.entity_size, &mut indices) {
			self.origin[0] = f32::from_bits(fields.origin[0]);
			self.origin[1] = f32::from_bits(fields.origin[1]);
			self.origin[2] = f32::from_bits(fields.origin[2]);

			self.color[0] = f32::from_bits(fields.highlight[0]);
			self.color[1] = f32::from_bits(fields.highlight[1]);
			self.color[2] = f32::from_bits(fields.highlight[2]);
		}
	}
}
impl serde::Serialize for DeathboxEntity {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::*;
		let mut state = serializer.serialize_struct("DeathBoxProp", 3)?;
		state.serialize_field(unsafe_obfstr!("entity_ptr"), &self.entity_ptr)?;
		state.serialize_field(unsafe_obfstr!("index"), &self.index)?;
		state.serialize_field(unsafe_obfstr!("class_name"), s!("DeathBoxProp"))?;
		state.end()
	}
}
