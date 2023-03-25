use crate::*;

mod highlight;
mod aimassist;
mod rcs;
#[cfg(feature = "dev")]
mod debugger;
mod scripts;
#[cfg(feature = "dev")]
mod projectile;
mod radar;
mod esp;
mod espdata;
mod espflags;
mod observers;
mod ring;
mod icons;

pub use self::icons::*;

#[derive(Default)]
pub struct CheatManager {
	highlight: highlight::Highlight,
	aimassist: aimassist::AimAssist,
	rcs: rcs::RCS,
	#[cfg(feature = "dev")]
	debugger: debugger::Debugger,
	scripts: scripts::Scripts,
	#[cfg(feature = "dev")]
	projectile: projectile::Projectile,
	radar: radar::Radar,
	esp: esp::ESP,
	obs: observers::Observers,
	ring: ring::RingDamage,
	pub show_outline: bool,
	pub full_bones: bool,
}
impl cvar::IVisit for CheatManager {
	#[inline(never)]
	fn visit(&mut self, f: &mut dyn FnMut(&mut dyn cvar::INode)) {
		f(&mut cvar::List(s!("highlight"), &mut self.highlight));
		f(&mut cvar::List(s!("aim"), &mut self.aimassist));
		f(&mut cvar::List(s!("rcs"), &mut self.rcs));
		#[cfg(feature = "dev")]
		f(&mut cvar::List(s!("dbg"), &mut self.debugger));
		f(&mut cvar::List(s!("scripts"), &mut self.scripts));
		#[cfg(feature = "dev")]
		f(&mut cvar::List(s!("projectile"), &mut self.projectile));
		f(&mut cvar::List(s!("radar"), &mut self.radar));
		f(&mut cvar::List(s!("esp"), &mut self.esp));
		f(&mut cvar::List(s!("obs"), &mut self.obs));
		f(&mut cvar::List(s!("ring"), &mut self.ring));
		f(&mut cvar::Property(s!("show_outline"), &mut self.show_outline, &false));
		f(&mut cvar::Property(s!("esp.full_bones"), &mut self.full_bones, &false));
	}
}
impl CheatManager {
	#[inline(never)]
	pub fn run(&mut self, api: &mut Api, ctx: &mut RunContext) {
		ctx.full_bones = self.full_bones;

		self.highlight.run(api, ctx);
		self.aimassist.run(api, ctx);
		self.rcs.run(api, ctx);
		self.scripts.run(api, ctx);
		#[cfg(feature = "dev")]
		self.projectile.run(api, ctx);
		#[cfg(feature = "dev")]
		self.debugger.run(api, ctx);

		// Render the overlay
		if api.r_begin(&mut ctx.screen) {
			if self.show_outline && !ctx.state.is_in_game() {
				api.r_rect(0.0, 0.0, ctx.screen[0] as f32, ctx.screen[1] as f32, vgc::sRGBA::TRANSPARENT, vgc::sRGBA!(Aqua));
			}
			self.obs.render(api, ctx);
			self.ring.render(api, ctx);
			self.radar.render(api, ctx);
			self.esp.render(api, ctx);
			api.r_end();
		}
	}
}

// fn rgba(color: [u8; 3], alpha: f32) -> u32 {
// 	vgc::sRGBA(color[0], color[1], color[2], (alpha * 255.0) as u8).pack()
// }
fn srgb2linear(color: [u8; 3]) -> [f32; 3] {
	[
		color[0] as f32 / 255.0,
		color[1] as f32 / 255.0,
		color[2] as f32 / 255.0,
	]
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InState {
	Default,
	Force,
	Press,
	Release,
}
impl InState {
	pub fn force(&mut self) {
		if let InState::Default = self {
			*self = InState::Force;
		}
	}
}

pub struct RunContext<'a> {
	pub process: &'a GameProcess,
	pub data: &'a GameData,
	pub state: &'a GameState,
	pub strings: &'a str,
	pub pool: &'a base::StringPool,

	pub screen: [i32; 2],
	pub full_bones: bool,

	pub jump: InState,
	pub duck: InState,
	pub attack: InState,
	pub reload: InState,
	pub inuse: InState,
	pub forward: InState,
}
impl<'a> RunContext<'a> {
	pub fn new(process: &'a GameProcess, data: &'a GameData, state: &'a GameState, strings: &'a str, pool: &'a base::StringPool) -> RunContext<'a> {
		RunContext {
			process, data, state, strings, pool,
			screen: [0, 0], // Resolved later
			full_bones: false,
			jump: InState::Default,
			duck: InState::Default,
			attack: InState::Default,
			reload: InState::Default,
			inuse: InState::Default,
			forward: InState::Default,
		}
	}
	pub fn entity_check(&self, api: &mut Api, index: u32, entity_ptr: sdk::Ptr) -> bool {
		let address = self.process.base.field::<[sdk::CEntInfo]>(self.data.entity_list);
		if let Ok(ent_info) = api.vm_read(address.at(index as usize)) {
			return ent_info.pEntity == entity_ptr;
		}
		return false;
	}
	pub fn rapidfire(&self) -> bool {
		// Tap at the rate of interval per tick
		(self.state.time / self.state.client.interval_per_tick as f64) as i64 & 1 != 0
	}
	/// Select the desired weapon slot.
	///
	/// 0 = primary, 1 = secondary, 2 = melee
	pub fn select_slot(&self, api: &mut Api, slot: u8) {
		if self.data.input == 0 {
			return;
		}
		let ptr = self.process.base.field(self.data.input + self.data.input_selected_slot);
		let bytes = [slot, 0, 0, 0, 0, 0, 0, 0, 1];
		let _ = api.vm_write(ptr, &bytes);
	}
	pub fn post(&mut self, api: &mut Api) {
		// If any button is not default
		fn any_button(this: &RunContext) -> bool {
			if !matches!(this.jump, InState::Default) { return true; }
			if !matches!(this.duck, InState::Default) { return true; }
			if !matches!(this.attack, InState::Default) { return true; }
			if !matches!(this.reload, InState::Default) { return true; }
			if !matches!(this.inuse, InState::Default) { return true; }
			if !matches!(this.forward, InState::Default) { return true; }
			return false;
		}
		if !any_button(self) {
			return;
		}

		// Read the current button state
		let mut buttons = state::Buttons::default();
		buttons.read(api, self.process, self.data);

		// Update their state if not what we expect
		#[inline(never)]
		pub fn post(api: &mut Api, process: &GameProcess, address: u32, button: &sdk::kbutton_t, state: InState, add: u32) {
			if let InState::Default = state {
				return;
			}
			let value = if let InState::Press = state { add + 1 }
			else if let InState::Release = state { add }
			else if button.down[0] == 0 && button.down[1] == 0 { add }
			else { add + 1 };
			if button.state != value {
				let _ = api.vm_write(process.base.field(address + 8), &value);
			}
		}
		post(api, self.process, self.data.in_jump, &buttons.in_jump, self.jump, 4);
		post(api, self.process, self.data.in_duck, &buttons.in_duck, self.duck, 4);
		post(api, self.process, self.data.in_attack, &buttons.in_attack, self.attack, 4);
		post(api, self.process, self.data.in_reload, &buttons.in_reload, self.reload, 4);
		post(api, self.process, self.data.in_use, &buttons.in_use, self.inuse, 4);
		post(api, self.process, self.data.in_forward, &buttons.in_forward, self.forward, 0);
	}
	pub fn world_to_screen(&self, v: [f32; 3], clip: bool) -> Option<[f32; 2]> {
		self.state.world_to_screen(v, &self.screen, clip)
	}
	pub fn angles_to_screen(&self, a: [f32; 3], clip: bool) -> Option<[f32; 2]> {
		let local = self.state.local_player()?;
		let dir = sdk::qvec(a);
		let point = sdk::add(local.camera_origin, sdk::muls(dir, 1000.0));
		self.world_to_screen(point, clip)
	}
	pub fn camera_origin(&self) -> [f32; 3] {
		let Some(local) = self.state.local_player() else { return [0.0; 3] };
		local.camera_origin
	}
}
