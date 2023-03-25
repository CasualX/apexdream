use std::str;
use super::*;

#[derive(Default)]
pub struct BaseEntity {
	pub entity_ptr: sdk::Ptr,
	pub index: u32,
	pub client_class_rva: u32,
	pub network_name: [u8; 32],
	pub origin: [f32; 3],
	// pub signifier_name: [u8; 32],
	pub model_name: ModelName,
}
impl BaseEntity {
	pub fn new(entity_ptr: sdk::Ptr, index: u32, client_class_rva: u32) -> Box<dyn Entity> {
		Box::new(BaseEntity { entity_ptr, index, client_class_rva, ..BaseEntity::default() }) as Box<dyn Entity>
	}
	// pub fn signifier_name(&self) -> &str {
	// 	from_utf8_buf(&self.signifier_name).unwrap_or("")
	// }
	pub fn network_name(&self) -> &str {
		crate::base::from_utf8_buf(&self.network_name).unwrap_or("")
	}
}
impl Entity for BaseEntity {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_ref(&self) -> EntityRef<'_> {
		EntityRef::BaseEntity(self)
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
		let process = ctx.process;
		let data = ctx.data;
		if self.network_name[0] == 0 {
			let client_class_ptr = process.base.field::<sdk::ClientClass>(self.client_class_rva);
			if let Ok(client_class) = api.vm_read(client_class_ptr) {
				let _ = api.vm_read_into(client_class.pNetworkName, &mut self.network_name);
			}
		}
		let entity_ptr = self.entity_ptr;
		// let _ = process.vm_read_into(entity_ptr.field(data.entity_signifier_name + 9), &mut self.signifier_name);
		let _ = api.vm_read_into(entity_ptr.field(data.entity_origin), &mut self.origin);
		if let Ok(model_name_ptr) = api.vm_read(entity_ptr.field::<sdk::Ptr<[u8]>>(data.entity_model_name)) {
			self.model_name.update(api, model_name_ptr);
		}
	}
}
impl serde::Serialize for BaseEntity {
	#[inline(never)]
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::*;
		let mut state = serializer.serialize_struct(unsafe_obfstr!("BaseEntity"), 4)?;
		state.serialize_field(unsafe_obfstr!("index"), &self.index)?;
		// state.serialize_field(unsafe_obfstr!("signifier_name"), self.signifier_name())?;
		state.serialize_field(unsafe_obfstr!("network_name"), self.network_name())?;
		state.serialize_field(unsafe_obfstr!("origin"), &self.origin)?;
		state.end()
	}
}
