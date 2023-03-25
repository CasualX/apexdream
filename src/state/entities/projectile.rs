use super::*;

#[derive(Default)]
pub struct ProjectileEntity {
	pub entity_ptr: sdk::Ptr,
	pub entity_size: u32,
	pub index: u32,

	pub origin: [f32; 3],
	pub velocity: [f32; 3],

	pub team_num: i32,
	pub owner_entity: sdk::EHandle,

	pub weapon_data_is_set: bool,
	pub force_adjust_to_gun_barrel_disabled: bool,
	pub weapon_class_index: i32,
	pub destruction_distance: f32,
	pub pass_through_depth_total: i32,
	pub mod_bitfield: u32,
	pub override_mods: u32,
	pub weapon_source: i32,
	pub launch_origin: [f32; 3],
}
impl ProjectileEntity {
	pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
		let entity_size = cc.ClassSize;
		Box::new(ProjectileEntity { entity_ptr, entity_size, index, ..ProjectileEntity::default() })
	}
}
impl Entity for ProjectileEntity {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_ref(&self) -> EntityRef<'_> {
		EntityRef::Projectile(self)
	}
	fn is_serialized(&self) -> bool {
		false
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
			origin: [u32; 3],
			velocity: [u32; 3],
			team_num: u32,
			owner_entity: u32,
			projectile: [u32; 10],
		}

		let data = ctx.data;
		let mut indices = Indices {
			origin: [
				data.entity_origin + 0,
				data.entity_origin + 4,
				data.entity_origin + 8],
			velocity: [
				data.entity_velocity + 0,
				data.entity_velocity + 4,
				data.entity_velocity + 8],
			team_num: data.entity_team_num,
			owner_entity: data.entity_owner_entity,
			projectile: [
				data.projectile + 0x00,
				data.projectile + 0x04,
				data.projectile + 0x08,
				data.projectile + 0x0c,
				data.projectile + 0x10,
				data.projectile + 0x14,
				data.projectile + 0x28,
				data.projectile + 0x44 + 0,
				data.projectile + 0x44 + 4,
				data.projectile + 0x44 + 8],
		};

		if let Ok(fields) = api.vm_gatherd(self.entity_ptr, self.entity_size, &mut indices) {
			self.origin[0] = f32::from_bits(fields.origin[0]);
			self.origin[1] = f32::from_bits(fields.origin[1]);
			self.origin[2] = f32::from_bits(fields.origin[2]);
			self.velocity[0] = f32::from_bits(fields.velocity[0]);
			self.velocity[1] = f32::from_bits(fields.velocity[1]);
			self.velocity[2] = f32::from_bits(fields.velocity[2]);
			self.team_num = fields.team_num as i32;
			self.owner_entity = sdk::EHandle::from(fields.owner_entity);
			self.weapon_data_is_set = fields.projectile[0] & 0xff != 0;
			self.force_adjust_to_gun_barrel_disabled = fields.projectile[0] & 0xff00 != 0;
			self.weapon_class_index = fields.projectile[1] as u16 as i32;
			self.destruction_distance = f32::from_bits(fields.projectile[2]);
			self.pass_through_depth_total = fields.projectile[3] as i32;
			self.mod_bitfield = fields.projectile[4];
			self.override_mods = fields.projectile[5];
			self.weapon_source = fields.projectile[6] as i32;
			self.launch_origin[0] = f32::from_bits(fields.projectile[7]);
			self.launch_origin[1] = f32::from_bits(fields.projectile[8]);
			self.launch_origin[2] = f32::from_bits(fields.projectile[9]);
		}
	}
}
