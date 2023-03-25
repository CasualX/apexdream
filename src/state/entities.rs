#![allow(dead_code)]

use std::any::Any;
use super::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct EntityInfo {
	pub entity_ptr: sdk::Ptr,
	pub index: usize,
	pub handle: sdk::EHandle,
	pub rate: u32,
}

pub trait Entity: Any {
	fn as_any(&self) -> &dyn Any;
	fn as_ref(&self) -> EntityRef<'_>;
	fn is_serialized(&self) -> bool;
	fn get_info(&self) -> EntityInfo;
	fn update(&mut self, api: &mut Api, ctx: &UpdateContext);
	fn post(&mut self, _api: &mut Api, _ctx: &UpdateContext, _state: &GameState) {}
}

#[derive(Copy, Clone)]
pub enum EntityRef<'a> {
	BaseEntity(&'a BaseEntity),
	BaseNPC(&'a BaseNPCEntity),
	World(&'a WorldEntity),
	Player(&'a PlayerEntity),
	WeaponX(&'a WeaponXEntity),
	Loot(&'a LootEntity),
	Waypoint(&'a WaypointEntity),
	Vehicle(&'a VehicleEntity),
	Deathbox(&'a DeathboxEntity),
	Animating(&'a AnimatingEntity),
	Projectile(&'a ProjectileEntity),
	ScriptNetData(&'a ScriptNetDataEntity),
}
impl serde::Serialize for EntityRef<'_> {
	#[inline(never)]
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		match self {
			EntityRef::BaseEntity(base) => base.serialize(serializer),
			EntityRef::BaseNPC(npc) => npc.serialize(serializer),
			EntityRef::World(world) => world.serialize(serializer),
			EntityRef::Player(player) => player.serialize(serializer),
			EntityRef::WeaponX(weapon) => weapon.serialize(serializer),
			EntityRef::Loot(loot) => loot.serialize(serializer),
			EntityRef::Waypoint(waypoint) => waypoint.serialize(serializer),
			EntityRef::Vehicle(vehicle) => vehicle.serialize(serializer),
			EntityRef::Deathbox(deathbox) => deathbox.serialize(serializer),
			EntityRef::Animating(animating) => animating.serialize(serializer),
			EntityRef::Projectile(projectile) => projectile.serialize(serializer),
			EntityRef::ScriptNetData(snd) => snd.serialize(serializer),
		}
	}
}
impl serde::Serialize for dyn Entity + '_ {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.as_ref().serialize(serializer)
	}
}
impl EntityRef<'_> {
	pub fn get_type_name(self, buf: &mut [u8; 32]) -> &str {
		match self {
			EntityRef::BaseEntity(_) => s!(buf <- "BaseEntity"),
			EntityRef::BaseNPC(_) => s!(buf <- "BaseNPC"),
			EntityRef::World(_) => s!(buf <- "World"),
			EntityRef::Player(_) => s!(buf <- "Player"),
			EntityRef::WeaponX(_) => s!(buf <- "WeaponX"),
			EntityRef::Loot(_) => s!(buf <- "Loot"),
			EntityRef::Waypoint(_) => s!(buf <- "Waypoint"),
			EntityRef::Vehicle(_) => s!(buf <- "Vehicle"),
			EntityRef::Deathbox(_) => s!(buf <- "Deathbox"),
			EntityRef::Animating(_) => s!(buf <- "Animating"),
			EntityRef::Projectile(_) => s!(buf <- "Projectile"),
			EntityRef::ScriptNetData(_) => s!(buf <- "ScriptNetData"),
		}
	}
}

mod base;
mod npc;
mod world;
mod player;
mod weaponx;
mod loot;
mod waypoint;
mod vehicle;
mod deathbox;
mod animating;
mod projectile;
mod scriptnetdata;

pub use self::base::BaseEntity;
pub use self::npc::BaseNPCEntity;
pub use self::world::WorldEntity;
pub use self::player::PlayerEntity;
pub use self::weaponx::WeaponXEntity;
pub use self::loot::LootEntity;
pub use self::waypoint::WaypointEntity;
pub use self::vehicle::VehicleEntity;
pub use self::deathbox::DeathboxEntity;
pub use self::animating::AnimatingEntity;
pub use self::projectile::ProjectileEntity;
pub use self::scriptnetdata::ScriptNetDataEntity;

mod utils;
pub use self::utils::BoneArray;

#[derive(Clone, Default)]
pub struct ModelName {
	pub ptr: sdk::Ptr<[u8]>,
	pub string: String,
	pub hash: sdk::ModelName,
}
impl ModelName {
	pub fn update(&mut self, api: &mut Api, model_name_ptr: sdk::Ptr<[u8]>) -> bool {
		// Update when pointer changes
		if model_name_ptr != self.ptr {
			self.string.clear();
			self.hash = Default::default();
			if !model_name_ptr.is_null() {
				let mut model_name = [0u8; 128];
				if let Ok(model_name) = api.vm_read_cstr(model_name_ptr, &mut model_name) {
					self.string.push_str(model_name);
					self.string.make_ascii_lowercase(); // Keep everything consistently lower cased
					self.hash = sdk::ModelName(crate::hash(&self.string));
					return true;
				}
			}
		}
		return false;
	}
}

#[derive(Copy, Clone, Debug)]
pub struct HitSphere {
	pub bone: i32,
	pub radius: f32,
}
