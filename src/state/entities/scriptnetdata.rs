use super::*;

const SIZE: usize = 32;

#[derive(Default)]
pub struct ScriptNetDataEntity {
	pub entity_ptr: sdk::Ptr,
	pub entity_size: u32,
	pub index: u32,
	recv_table: sdk::Ptr<sdk::RecvTableRedux>,

	// Offsets for [m_bools, m_ranges, m_int32s, m_times, m_entities] respectively
	offsets: [u32; 5],

	initialized: u8,

	// Script data related to local player
	local_player: bool,

	pub bools: [u8; SIZE],
	pub ranges: [i16; SIZE],
	pub ints: [i32; SIZE],
	pub times: [f32; SIZE],
	pub ents: [sdk::EHandle; SIZE],
}

impl ScriptNetDataEntity {
	pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
		let entity_size = cc.ClassSize;
		let recv_table = cc.pRecvTable.cast();
		Box::new(ScriptNetDataEntity { entity_ptr, entity_size, index, recv_table, ..Default::default() })
	}

	pub fn bools(&self) -> &[u8] {
		let len = (self.offsets[1] - self.offsets[0]) as usize;
		&self.bools[..usize::min(SIZE, len)]
	}
	pub fn shorts(&self) -> &[i16] {
		let len = (self.offsets[2] - self.offsets[1]) as usize / 2;
		&self.ranges[..usize::min(SIZE, len)]
	}
	pub fn ints(&self) -> &[i32] {
		let len = (self.offsets[3] - self.offsets[2]) as usize / 4;
		&self.ints[..usize::min(SIZE, len)]
	}
	pub fn floats(&self) -> &[f32] {
		let len = (self.offsets[4] - self.offsets[3]) as usize / 4;
		&self.times[..usize::min(SIZE, len)]
	}
	pub fn ents(&self) -> &[sdk::EHandle] {
		let len = (self.entity_size - self.offsets[4]) as usize / 4;
		&self.ents[..usize::min(SIZE, len)]
	}

	fn init(&mut self, api: &mut Api) -> bool {
		if self.initialized == 1 {
			return true;
		}
		if self.initialized == 2 {
			return false;
		}
		self.initialized = 2;

		let Ok(recv_table) = api.vm_read(self.recv_table) else { return false };
		if recv_table.num_props > 100 {
			return false;
		}

		let mut name_buf = [0u8; 64];
		let mut count = 0;
		for i in 0..11 {
			let Ok(prop_ptr) = api.vm_read(recv_table.props.at(i)) else { return false };
			let Ok(prop) = api.vm_read(prop_ptr) else { return false };
			let Ok(name) = api.vm_read_cstr(prop.name, &mut name_buf) else { return false };
			let name = hash(name);

			if name == hash!("m_bools[0]") {
				self.offsets[0] = prop.offset as u32;
				count += 1;
			}
			else if name == hash!("m_ranges[0]") {
				self.offsets[1] = prop.offset as u32;
				count += 1;
			}
			else if name == hash!("m_int32s[0]") {
				self.offsets[2] = prop.offset as u32;
				count += 1;
			}
			else if name == hash!("m_times[0]") {
				self.offsets[3] = prop.offset as u32;
				count += 1;
			}
			else if name == hash!("m_entities[0]") {
				self.offsets[4] = prop.offset as u32;
				count += 1;
			}
		}

		if count != 5 {
			return false;
		}

		let indices = fmtools::join(", ", self.offsets.iter().map(|idx| f!(move {idx:#x})));
		api.log(f!("ScriptNetData indices=["{indices}"]"));
		self.initialized = 1;
		return true;
	}
}

impl Entity for ScriptNetDataEntity {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_ref(&self) -> EntityRef<'_> {
		EntityRef::ScriptNetData(self)
	}
	fn is_serialized(&self) -> bool {
		false
	}
	fn get_info(&self) -> EntityInfo {
		EntityInfo {
			entity_ptr: self.entity_ptr,
			handle: sdk::EHandle::from(self.index),
			index: self.index as usize,
			rate: if self.local_player { 1 } else { 64 },
		}
	}
	fn update(&mut self, api: &mut Api, _ctx: &UpdateContext) {
		if !self.local_player {
			return;
		}

		if !self.init(api) {
			return;
		}

		let mut buf = [0u32; 500 / 4];

		let start_offset = self.offsets.iter().cloned().min().unwrap_or(0);

		let Some(buf) = dataview::bytes_mut(&mut buf).get_mut(..(self.entity_size - start_offset) as usize) else { return };

		// Read into buffer
		let _ = api.vm_read_into(self.entity_ptr.field(start_offset), buf);

		let view = dataview::DataView::from(buf);
		for i in 0..SIZE {
			let bool_offset = (self.offsets[0] - start_offset) as usize + i;
			self.bools[i] = view.try_read::<u8>(bool_offset).unwrap_or(0);

			let range_offset = (self.offsets[1] - start_offset) as usize + i * 2;
			self.ranges[i] = view.try_read::<i16>(range_offset).unwrap_or(0);

			let int_offset = (self.offsets[2] - start_offset) as usize + i * 4;
			self.ints[i] = view.try_read::<i32>(int_offset).unwrap_or(0);

			let time_offset = (self.offsets[3] - start_offset) as usize + i * 4;
			self.times[i] = view.try_read::<f32>(time_offset).unwrap_or(0.0);

			let ent_offset = (self.offsets[4] - start_offset) as usize + i * 4;
			self.ents[i] = view.try_read::<sdk::EHandle>(ent_offset).unwrap_or_default();
		}
	}
	fn post(&mut self, _api: &mut Api, _ctx: &UpdateContext, state: &GameState) {
		self.local_player = false;
		let Some(local) = state.local_player() else { return };
		if local.script_net_data_exclusive.signed_index() == self.index as i32 {
			self.local_player = true;
		}
		if local.script_net_data_global.signed_index() == self.index as i32 {
			self.local_player = true;
		}
	}
}

impl serde::Serialize for ScriptNetDataEntity {
	#[inline(never)]
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::*;
		let state = serializer.serialize_struct(unsafe_obfstr!("ScriptNetData"), 0)?;
		state.end()
	}
}
