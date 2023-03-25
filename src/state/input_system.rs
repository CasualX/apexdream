use super::*;

use bitset_core::BitSet;

#[derive(Default)]
pub struct InputSystem {
	pub button_state: [u32; 4],
}
impl InputSystem {
	pub fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
		if ctx.ticked(2, 0) {
			let _ = api.vm_read_into(ctx.process.base.field(ctx.data.input_system + ctx.data.input_button_state), &mut self.button_state);
		}
	}
}
impl serde::Serialize for InputSystem {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::*;
		let mut state = serializer.serialize_struct(unsafe_obfstr!("InputSystem"), 1)?;
		state.serialize_field(unsafe_obfstr!("button_state"), &self.button_state[..])?;
		state.end()
	}
}

//----------------------------------------------------------------
// GameState helpers

#[allow(dead_code)]
impl super::GameState {
	/// Tests if the given button is pressed.
	pub fn is_button_down(&self, button_code: i32) -> bool {
		let button_state = &self.input_system.button_state;
		if button_code as usize >= button_state.bit_len() {
			return false;
		}
		button_state.bit_test(button_code as usize)
	}
	/// Tests if any button is pressed.
	pub fn is_any_button_down(&self) -> bool {
		self.input_system.button_state.bit_any()
	}
}
