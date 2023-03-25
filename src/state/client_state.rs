use super::*;

#[derive(Default)]
pub struct ClientState {
	pub signon_state: i32,
	pub level_hash: u32,
	pub level_name: String,

	pub local_entity: sdk::EHandle,
	pub curtime: f32, // Time used for comparison against last_visible_time
	pub interval_per_tick: f32,
	pub view_render: sdk::Ptr,
	pub view_matrix_ptr: sdk::Ptr<[f32; 16]>,
	pub view_matrix: [f32; 16],
}
impl ClientState {
	pub fn update(&mut self, api: &mut Api, ctx: &mut UpdateContext) {
		let process = ctx.process;
		let data = ctx.data;

		// Connection signon state
		if ctx.ticked(25, 24) {
			if let Ok(signon_state) = api.vm_read::<i32>(process.base.field(data.client_state + data.client_signon_state)) {
				ctx.connected = self.signon_state != sdk::SIGNONSTATE_FULL && signon_state == sdk::SIGNONSTATE_FULL;
				self.signon_state = signon_state;
			}
		}

		// Current level name
		if ctx.connected {
			self.level_hash = 0;
			self.level_name.clear();

			let mut level_name = [0u8; 0x40];
			if let Ok(level_name) = api.vm_read_cstr(process.base.field(data.client_state + data.client_level_name), &mut level_name) {
				self.level_hash = hash(level_name);
				self.level_name.push_str(level_name);
			}
		}

		// Update full entity list only in lobby and firing range
		// ctx.full_entlist =
		// 	self.level_hash == hash!("mp_lobby") ||
		// 	self.level_hash == hash!("mp_rr_canyonlands_staging");

		// Local player entity handle
		if ctx.connected || ctx.ticked(100, 11) {
			let _ = api.vm_read_into(process.base.field(data.local_entity_handle), &mut self.local_entity);
		}
		ctx.local_entity = self.local_entity;

		// Globals
		if let Ok(globals) = api.vm_read::<sdk::CGlobalVars>(process.base.field(data.global_vars)) {
			self.curtime = globals.curtime;
			self.interval_per_tick = 1.0 / 20.0;
		}

		// ViewMatrix
		if ctx.connected || ctx.ticked(25, 6) {
			self.view_render = sdk::Ptr::NULL;
			let _ = api.vm_read_into(process.base.field(data.view_render), &mut self.view_render);
		}
		if !self.view_render.is_null() && (ctx.connected || ctx.ticked(25, 14)) {
			self.view_matrix_ptr = sdk::Ptr::NULL;
			let _ = api.vm_read_into(self.view_render.field(data.view_matrix), &mut self.view_matrix_ptr);
		}
		if !self.view_matrix_ptr.is_null() {
			let _ = api.vm_read_into(self.view_matrix_ptr, &mut self.view_matrix);
		}
	}
}
impl GameState {
	pub fn is_in_game(&self) -> bool {
		return
			self.client.signon_state == sdk::SIGNONSTATE_FULL &&
			self.client.level_hash != hash!("mp_lobby");
	}
	pub fn is_firing_range(&self) -> bool {
		self.client.level_hash == hash!("mp_rr_canyonlands_staging")
	}
	pub fn world_to_screen(&self, v: [f32; 3], screen: &[i32; 2], clip: bool) -> Option<[f32; 2]> {
		let vmatrix = &self.client.view_matrix;

		let w = vmatrix[12] * v[0] + vmatrix[13] * v[1] + vmatrix[14] * v[2] + vmatrix[15];
		if w < 0.01 {
			return None;
		}

		let invw = 1.0 / w;
		let vx = (vmatrix[0] * v[0] + vmatrix[1] * v[1] + vmatrix[2] * v[2] + vmatrix[3]) * invw;
		let vy = (vmatrix[4] * v[0] + vmatrix[5] * v[1] + vmatrix[6] * v[2] + vmatrix[7]) * invw;

		// If the resulting coordinate is too far outside the screen bounds clip it manually
		if clip {
			if vx < -2.0 || vx > 2.0 || vy < -2.0 || vy > 2.0 {
				return None;
			}
		}

		let width = screen[0] as f32 * 0.5;
		let height = screen[1] as f32 * 0.5;

		let px = width + vx * width + 0.5;
		let py = height - vy * height + 0.5;
		Some([px, py])
	}
}
