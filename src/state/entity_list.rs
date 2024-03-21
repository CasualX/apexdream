use std::{mem, collections::HashMap};
use super::*;

pub struct EntityList {
	pub entities: Box<[Option<Box<dyn Entity>>]>,
	ent_info: Box<[sdk::CEntInfo]>,
	prev_info: Box<[sdk::CEntInfo]>,
	pub updates: u32,
	next_index: u32,
	gce: GetClientEntity,
}
impl Default for EntityList {
	fn default() -> EntityList {
		let mut entities = Vec::new();
		entities.resize_with(sdk::NUM_ENT_ENTRIES, || None);
		EntityList {
			entities: entities.into_boxed_slice(),
			ent_info: vec![sdk::CEntInfo::default(); sdk::NUM_ENT_ENTRIES].into_boxed_slice(),
			prev_info: vec![sdk::CEntInfo::default(); sdk::NUM_ENT_ENTRIES].into_boxed_slice(),
			updates: 0,
			next_index: 0,
			gce: GetClientEntity::default(),
		}
	}
}


impl EntityList {
	#[inline(never)]
	pub fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
		let process = ctx.process;

		self.updates = 0;

		// Update entity list in smaller chunks over time
		let count;
		if self.gce.config.full_entlist {
			self.next_index = 0;
			count = sdk::NUM_ENT_ENTRIES;
		}
		else {
			count = sdk::NUM_ENT_ENTRIES / 32;
		}

		let start = self.next_index as usize;
		let end = usize::min(start + count, sdk::NUM_ENT_ENTRIES);
		self.next_index = if end == sdk::NUM_ENT_ENTRIES { 0 } else { end as u32 };

		// Read a chunk of the game's ent info array
		if let Some(ent_info_slice) = self.ent_info.get_mut(start..end) {
			let _ = api.vm_read_into(process.base.field(ctx.data.entity_list + start as u32 * 32), ent_info_slice);
		}

		// Update the entities
		let prev_info = unsafe { self.prev_info.get_unchecked(..sdk::NUM_ENT_ENTRIES) };
		let ent_info = unsafe { self.ent_info.get_unchecked(..sdk::NUM_ENT_ENTRIES) };
		let entities = unsafe { self.entities.get_unchecked_mut(..sdk::NUM_ENT_ENTRIES) };
		for index in 0..sdk::NUM_ENT_ENTRIES {
			// If entity pointer has changed
			if index >= start && index < end && prev_info[index].pEntity != ent_info[index].pEntity {
				// Recreate the entity object with the correct type
				let entity_ptr = ent_info[index].pEntity;
				entities[index] = self.gce.create_entity(api, entity_ptr, index as u32);
				// Always update the entity when created
				if let Some(entity) = &mut entities[index] {
					entity.update(api, ctx);
					self.updates += 1;
				}
			}
			// Update the entity at their specified rate if we are tracking it
			else if let Some(entity) = &mut entities[index] {
				let rate = entity.get_info().rate;
				if ctx.ticked(rate, index as u32) {
					entity.update(api, ctx);
					self.updates += 1;
				}
			}
		}

		// If we reached the end, swap the entity infos
		if end == sdk::NUM_ENT_ENTRIES {
			mem::swap(&mut self.ent_info, &mut self.prev_info);
		}
	}
}

//----------------------------------------------------------------
// GetClientEntity

struct Config {
	log_errors: bool,
	log_uninteresting: bool,
	full_entlist: bool,
}
impl Default for Config {
	fn default() -> Self {
		Config {
			log_errors: false,
			log_uninteresting: false,
			full_entlist: false,
		}
	}
}

struct ClientClassData {
	client_class: sdk::ClientClass,
	name_hash: u32,
	name_buf: [u8; 52],
}

#[derive(Default)]
struct GetClientEntity {
	config: Config,
	lookup: HashMap<sdk::Ptr<[sdk::Ptr]>, ClientClassData>,
}

impl GetClientEntity {
	#[inline(never)]
	fn create_entity(&mut self, api: &mut Api, entity_ptr: sdk::Ptr, index: u32) -> Option<Box<dyn Entity>> {
		if entity_ptr.is_null() {
			return None;
		}
		// Filter out bad addresses (mainly from running the hack before the game has decrypted itself)
		if entity_ptr.into_raw() & 7 != 0 || entity_ptr.into_raw() >= (1 << 48) {
			return None;
		}

		// Borrowck error avoidance :)
		let log_uninteresting = self.config.log_uninteresting;

		// Get the entity type name
		let data = self.get_client_class(api, entity_ptr)?;

		match data.name_hash {
			sdk::CPlayer => {
				Some(PlayerEntity::new(entity_ptr, index, &data.client_class))
			},
			sdk::CPropSurvival => {
				Some(LootEntity::new(entity_ptr, index, &data.client_class))
			},
			sdk::CWeaponX => {
				Some(WeaponXEntity::new(entity_ptr, index, &data.client_class))
			},
			sdk::CWorld => {
				Some(WorldEntity::new(entity_ptr, index, &data.client_class))
			},
			sdk::CAI_BaseNPC => {
				Some(BaseNPCEntity::new(entity_ptr, index, &data.client_class))
			},
			// sdk::CPlayerWaypoint => {
			// 	Some(WaypointEntity::new(entity_ptr, index, &data.client_class))
			// },
			sdk::CPlayerVehicle => {
				Some(VehicleEntity::new(entity_ptr, index, &data.client_class))
			},
			sdk::CDeathBoxProp => {
				Some(DeathboxEntity::new(entity_ptr, index, &data.client_class))
			},
			sdk::CDynamicProp | sdk::CScriptProp | sdk::CPhysicsProp => {
				Some(AnimatingEntity::new(entity_ptr, index, &data.client_class))
			},
			sdk::CScriptNetData_SNDC_PLAYER_GLOBAL => {
				Some(ScriptNetDataEntity::new(entity_ptr, index, &data.client_class))
			},
			sdk::CScriptNetData_SNDC_PLAYER_EXCLUSIVE => {
				Some(ScriptNetDataEntity::new(entity_ptr, index, &data.client_class))
			},
			// sdk::CCrossbowBolt | sdk::CBaseGrenade => {
			// 	Some(ProjectileEntity::new(entity_ptr, index, &data.client_class))
			// },
			// _ => {
			// 	Some(BaseEntity::new(entity_ptr, index, &data.client_class))
			// },
			_ => {
				if log_uninteresting {
					let name = base::from_utf8_buf(&data.name_buf);
					api.log(f!("Uninteresting["{index}"]: "{name:?}));
				}
				None
			},
		}
	}

	#[inline(never)]
	fn get_client_class(&mut self, api: &mut Api, entity_ptr: sdk::Ptr) -> Option<&ClientClassData> {
		// Read the IClientNetworkable vtable at entity_ptr + 3 * 8
		let client_networkable: sdk::Ptr<[sdk::Ptr]> = match api.vm_read(entity_ptr.field(3 * 8)) {
			Ok(p) => p,
			Err(_) => {
				if self.config.log_errors {
					api.log(f!("get_client_class("{entity_ptr}"): IClientNetworkable"));
				}
				return None;
			}
		};

		// This can be null?!?
		if client_networkable.is_null() {
			return None;
		}

		// Aggressively cache these lookups
		if !self.lookup.contains_key(&client_networkable) {

			// Read the GetClientEntity function ptr
			let get_client_entity = match api.vm_read(client_networkable.at(3)) {
				Ok(pgce) => pgce,
				Err(_) => {
					if self.config.log_errors {
						api.log(f!("get_client_class("{entity_ptr}"): GetClientEntity {client_networkable="{client_networkable}"}"));
					}
					return None;
				}
			};

			// Read the offset out of the lea rax, offset instruction
			let offset = match api.vm_read::<i32>(get_client_entity.field(3)) {
				Ok(offset) => offset,
				Err(_) => {
					if self.config.log_errors {
						api.log(f!("get_client_class("{entity_ptr}"): lea rax, offset {get_client_entity="{get_client_entity}"}"));
					}
					return None;
				}
			};

			// Resolve relative offset
			let client_class_ptr = get_client_entity.offset((offset + 7) as i64);

			// Read ClientClass instance
			let client_class = match api.vm_read::<sdk::ClientClass>(client_class_ptr) {
				Ok(cc) => cc,
				Err(_) => {
					if self.config.log_errors {
						api.log(f!("get_client_class("{entity_ptr}"): ClientClass {get_client_entity="{get_client_entity}"}"));
					}
					return None;
				}
			};

			// FIXME! Figure out why CParticleSystem is horribly broken...
			if client_class.ClassID < 0 || client_class.ClassID > 500 {
				return None;
			}

			// Read pNetworkName
			let mut name_buf = [0u8; 52];
			let name = match api.vm_read_cstr(client_class.pNetworkName, &mut name_buf) {
				Ok(name) => name,
				Err(_) => {
					if self.config.log_errors {
						api.log(f!("get_client_class("{entity_ptr}"): pNetworkName {get_client_entity="{get_client_entity}", ClassID="{client_class.ClassID}"}"));
					}
					return None;
				}
			};

			// Cache the lookup
			let name_hash = hash(name);
			self.lookup.insert(client_networkable, ClientClassData { client_class, name_hash, name_buf });
		}

		self.lookup.get(&client_networkable)
	}
}

//----------------------------------------------------------------
// GameState helpers

#[allow(dead_code)]
impl super::GameState {
	/// Returns if an entity exists at the given index.
	pub fn is_entity(&self, handle: sdk::EHandle) -> bool {
		match handle.index() {
			Some(i) => self.entity_list.entities.get(i).is_some(),
			None => false,
		}
	}
	/// Returns the entity at the given index if it exists.
	pub fn entity(&self, handle: sdk::EHandle) -> Option<&dyn Entity> {
		let i = handle.index()?;
		let boxed = self.entity_list.entities.get(i)?;
		Some(&**boxed.as_ref()?)
	}
	/// Returns an Iterator over all valid entities.
	pub fn entities(&self) -> impl Clone + Iterator<Item = &dyn Entity> {
		self.entity_list.entities.iter()
			.filter_map(|x| x.as_ref().map(std::ops::Deref::deref))
	}
	/// Returns the entity at the given index if it exists and matches the given type.
	pub fn entity_as<T: Entity>(&self, handle: sdk::EHandle) -> Option<&T> {
		let i = handle.index()?;
		let boxed = self.entity_list.entities.get(i)?;
		let entity = &**boxed.as_ref()?;
		entity.as_any().downcast_ref()
	}
	/// Returns an Iterator over all entities of the given type.
	pub fn entities_as<T: Entity>(&self) -> impl Clone + Iterator<Item = &T> {
		self.entity_list.entities.iter().filter_map(|x| {
			x.as_ref().and_then(|e| e.as_any().downcast_ref())
		})
	}
	/// Returns an Iterator over the player entities.
	pub fn players(&self) -> impl Clone + Iterator<Item = &PlayerEntity> {
		let len = self.entity_list.entities.len().min(sdk::MAX_PLAYERS + 1);
		self.entity_list.entities[..len].iter().filter_map(|x|
			x.as_ref().and_then(|e| e.as_any().downcast_ref()))
	}
	/// Given the entity pointer, find its entity index.
	pub fn entity_index(&self, entity_ptr: sdk::Ptr) -> Option<usize> {
		if entity_ptr.is_null() {
			return None;
		}
		self.entity_list.ent_info.iter()
			.position(|ent_info| ent_info.pEntity == entity_ptr)
	}
	/// Returns the local player entity if it exists.
	pub fn local_player(&self) -> Option<&PlayerEntity> {
		self.entity_as(self.client.local_entity)
	}
	/// Returns the local player if alive, else the player being observed, if any.
	pub fn camera_player(&self) -> Option<&PlayerEntity> {
		match self.local_player() {
			Some(local) if local.is_alive() => { Some(local) }
			Some(local) => self.entity_as(local.observer_target),
			None => None,
		}
	}
	pub fn world_entity(&self) -> Option<&WorldEntity> {
		self.entity_as(0.into())
	}
}
