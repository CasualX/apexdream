use bitset_core::BitSet;
use crate::*;

#[derive(Debug)]
struct Style {
	enable: bool,
	inside: u8,
	outline: u8,
	radius: u8,
	opacity: u8,
	visible: bool,
	post_process: bool,
	brightness: f32,
}
impl cvar::IVisit for Style {
	#[inline(never)]
	fn visit(&mut self, f: &mut dyn FnMut(&mut dyn cvar::INode)) {
		f(&mut cvar::Property(s!("enable"), &mut self.enable, &true));
		f(&mut cvar::Property(s!("inside"), &mut self.inside, &0));
		f(&mut cvar::Property(s!("outline"), &mut self.outline, &0));
		f(&mut cvar::Property(s!("radius"), &mut self.radius, &0));
		f(&mut cvar::Property(s!("opacity"), &mut self.opacity, &0));
		f(&mut cvar::Property(s!("visible"), &mut self.visible, &true));
		f(&mut cvar::Property(s!("post_process"), &mut self.post_process, &false));
		f(&mut cvar::Property(s!("brightness"), &mut self.brightness, &1.0));
	}
}
impl Style {
	const fn to_bits(&self) -> sdk::HighlightBits {
		sdk::HighlightBits::new(self.inside, self.outline, self.radius, self.opacity, self.visible, self.post_process)
	}
}

#[derive(Debug)]
struct Config {
	enable: bool,
	enable_loot: bool,
	enable_deathbox: bool,
	enable_animating: bool,
	debug: bool,

	player1: Style, // Visible player
	player2: Style, // Hidden player
	player3: Style, // Downed player
	npc1: Style, // Visible NPC
	npc2: Style, // Hidden NPC
	loot1: Style, // Relevant loot
	loot2: Style, // Irrelevant loot
	deathbox: Style, // Deathbox
	disable: Style,
	object: Style,
	loot_bin: Style,
}

impl Default for Config {
	fn default() -> Config {
		Config {
			enable: false,
			enable_loot: true,
			enable_deathbox: true,
			enable_animating: false,
			debug: false,
			player1: Style { enable: true, inside: 10, outline: 125, radius: 32, opacity: 13, visible: true, post_process: false, brightness: 1.0 },
			player2: Style { enable: true, inside: 10, outline: 125, radius: 32, opacity: 13, visible: true, post_process: false, brightness: 1.0 },
			player3: Style { enable: true, inside: 133, outline: 125, radius: 32, opacity: 13, visible: true, post_process: false, brightness: 0.5 },
			npc1: Style { enable: true, inside: 10, outline: 125, radius: 32, opacity: 7, visible: true, post_process: false, brightness: 1.0 },
			npc2: Style { enable: true, inside: 0, outline: 125, radius: 32, opacity: 7, visible: true, post_process: false, brightness: 0.4 },
			loot1: Style { enable: true, inside: 126, outline: 125, radius: 32, opacity: 27, visible: true, post_process: false, brightness: 1.0 },
			loot2: Style { enable: false, inside: 0, outline: 125, radius: 32, opacity: 27, visible: true, post_process: false, brightness: 1.0 },
			deathbox: Style { enable: true, inside: 112, outline: 125, radius: 32, opacity: 28, visible: true, post_process: false, brightness: 1.0 },
			disable: Style { enable: false, inside: 0, outline: 0, radius: 32, opacity: 0, visible: true, post_process: false, brightness: 1.0 },
			object: Style { enable: true, inside: 136, outline: 125, radius: 32, opacity: 0, visible: true, post_process: false, brightness: 1.0 },
			loot_bin: Style { enable: false, inside: 136, outline: 125, radius: 32, opacity: 0, visible: true, post_process: false, brightness: 1.0 },
		}
	}
}

impl cvar::IVisit for Highlight {
	#[inline(never)]
	fn visit(&mut self, f: &mut dyn FnMut(&mut dyn cvar::INode)) {
		let default = Config::default();
		f(&mut cvar::Property(s!("enable"), &mut self.config.enable, &default.enable));
		f(&mut cvar::Property(s!("enable.loot"), &mut self.config.enable_loot, &default.enable_loot));
		f(&mut cvar::Property(s!("enable.deathbox"), &mut self.config.enable_deathbox, &default.enable_deathbox));
		f(&mut cvar::Property(s!("enable.animating"), &mut self.config.enable_animating, &default.enable_animating));
		f(&mut cvar::Property(s!("debug"), &mut self.config.debug, &default.debug));
		f(&mut cvar::List(s!("player[vis]"), &mut self.config.player1));
		f(&mut cvar::List(s!("player[los]"), &mut self.config.player2));
		f(&mut cvar::List(s!("player[down]"), &mut self.config.player3));
		f(&mut cvar::List(s!("npc[vis]"), &mut self.config.npc1));
		f(&mut cvar::List(s!("npc[los]"), &mut self.config.npc2));
		f(&mut cvar::List(s!("loot[rel]"), &mut self.config.loot1));
		f(&mut cvar::List(s!("loot[ir]"), &mut self.config.loot2));
		f(&mut cvar::List(s!("deathbox"), &mut self.config.deathbox));
		f(&mut cvar::List(s!("object"), &mut self.config.object));
		f(&mut cvar::List(s!("loot_bin"), &mut self.config.loot_bin));
	}
}

#[derive(Debug, Default)]
pub struct Highlight {
	config: Config,
	offset: u32,
}

impl Highlight {
	#[inline(never)]
	pub fn run(&mut self, api: &mut Api, ctx: &mut RunContext) {
		if !self.config.enable {
			return;
		}

		let state = ctx.state;

		// Make the wallhacks relevant to the player being spectated
		let Some(camera) = state.camera_player() else { return };

		// Precalculate the set of desired items
		let desired_items = state.desired_items(camera);

		// Apply highlight to entities
		for entity in state.entities() {
			match entity.as_ref() {
				EntityRef::Player(player) => self.highlight_player(api, ctx, player, camera),
				EntityRef::BaseNPC(npc) => self.highlight_npc(api, ctx, npc, camera),
				EntityRef::Loot(loot) if self.config.enable_loot => self.highlight_prop(api, ctx, loot, camera, &desired_items),
				EntityRef::Deathbox(deathbox) if self.config.enable_deathbox => self.highlight_deathbox(api, ctx, deathbox, camera),
				EntityRef::Animating(anim) if self.config.enable_animating => self.highlight_animating(api, ctx, anim, camera),
				_ => (),
			}
		}

		// Update in waves at certain indices
		self.offset = self.offset.wrapping_add(1);
	}
	fn highlight_player(&mut self, api: &mut Api, ctx: &mut RunContext, player: &PlayerEntity, camera: &PlayerEntity) {
		if !is_in_fov(camera, &player.origin) {
			return;
		}

		// Do not glow team members
		if !self.config.debug && ctx.state.is_same_team(player, camera) {
			return;
		}

		// Choose the style
		let style = if player.is_downed() {
			&self.config.player3
		}
		else if !player.is_visible {
			&self.config.player2
		}
		else {
			&self.config.player1
		};

		highlight(api, ctx, player.index, player.entity_ptr, style, &super::srgb2linear(player.team_color));
	}
	fn highlight_npc(&mut self, api: &mut Api, ctx: &mut RunContext, npc: &BaseNPCEntity, camera: &PlayerEntity) {
		if !is_in_fov(camera, &npc.origin) {
			return;
		}

		// Choose the style
		let style = if !npc.is_visible {
			&self.config.npc2
		}
		else {
			&self.config.npc1
		};

		highlight(api, ctx, npc.index, npc.entity_ptr, style, &[1.0, 0.25, 0.0]);
	}
	// fn highlight_anim(&mut self, ctx: &mut RunContext, anim: &AnimatingEntity) {
	// 	let ptr: sdk::Ptr<sdk::HighlightSettings> = anim.entity_ptr.field(ctx.data.entity_highlight);
	// 	if let Ok(mut highlight) = ctx.process.vm_read(ptr) {
	// 		if ctx.entity_check(anim.index, anim.entity_ptr) {
	// 			let idx = sdk::HIGHLIGHT_CONTEXT_SONAR as usize;
	// 			highlight.current_context_id = idx as i32;
	// 			highlight.client_context_id = idx as i32;
	// 			highlight.server_context_id = idx as i32;
	// 			highlight.visibility_type = sdk::HIGHLIGHT_VIS_FORCE_ON;
	// 			highlight.params[idx].color = [1.0, 0.25, 0.0];
	// 			highlight.server_function_bits[idx] = sdk::HighlightBits::new(12, 169, 32, 7, true, false);
	// 			highlight.client_function_bits[idx] = sdk::HighlightBits::new(12, 169, 32, 7, true, false);
	// 			let _ = ctx.process.vm_write(ptr, &highlight);
	// 		}
	// 	}
	// }
	fn highlight_prop(&mut self, api: &mut Api, ctx: &mut RunContext, loot: &LootEntity, camera: &PlayerEntity, desired_items: &sdk::ItemSet) {
		if !is_in_fov(camera, &loot.origin) {
			return;
		}

		// Update in waves at certain indices
		if loot.index % 32 != self.offset % 32 {
			return;
		}

		// Get the highlight style depending on whether the loot is relevant
		let index = loot.known_item.0 as usize;
		let relevant = index < desired_items.bit_len() && desired_items.bit_test(index);

		// Highlight some important items always
		let important = match loot.known_item {
			sdk::ItemId::Keycard => true,
			sdk::ItemId::MrvnArm => true,
			_ => false,
		};

		// Choose the style
		let style = if important {
			&self.config.loot1
		}
		else if !ctx.state.player_is_melee(camera) {
			&self.config.disable
		}
		else if relevant {
			&self.config.loot1
		}
		else {
			&self.config.loot2
		};

		highlight(api, ctx, loot.index, loot.entity_ptr, style, &loot.color);
	}
	fn highlight_deathbox(&mut self, api: &mut Api, ctx: &mut RunContext, deathbox: &DeathboxEntity, camera: &PlayerEntity) {
		if !is_in_fov(camera, &deathbox.origin) {
			return;
		}

		// Update in waves at certain indices
		if deathbox.index % 4 != self.offset % 4 {
			return;
		}

		// Choose the style
		let style = if !ctx.state.player_is_melee(camera) {
			&self.config.disable
		}
		else {
			&self.config.deathbox
		};

		highlight(api, ctx, deathbox.index, deathbox.entity_ptr, style, &deathbox.color);
	}
	fn highlight_animating(&mut self, api: &mut Api, ctx: &mut RunContext, anim: &AnimatingEntity, camera: &PlayerEntity) {
		if !is_in_fov(camera, &anim.origin) {
			return;
		}

		// Update in waves at certain indices
		if anim.index % 4 != self.offset % 4 {
			return;
		}

		// Ignore highlight on objects owned by a member of our team
		// Since that is probably already highlighted by the game
		if let Some(owner) = ctx.state.entity_as::<PlayerEntity>(anim.owner_entity) {
			if owner.team_num == camera.team_num {
				return;
			}
		}

		let style = match anim.model_name.hash {
			sdk::ModelName::COVER_WALL => &self.config.object,
			sdk::ModelName::ELECTRIC_FENCE => &self.config.object,
			sdk::ModelName::GAS_TANK => &self.config.object,
			sdk::ModelName::HOLO_STAND => &self.config.object,
			sdk::ModelName::JUMP_PAD => &self.config.object,
			sdk::ModelName::LOOT_BIN => &self.config.loot_bin,
			sdk::ModelName::LOOT_DRONE => &self.config.object,
			sdk::ModelName::LOOT_SPHERE => &self.config.object,
			sdk::ModelName::LOOT_TICK => &self.config.object,
			sdk::ModelName::ROCKETS_PROJECTILE => &self.config.object,
			sdk::ModelName::TOTEM => &self.config.object,
			sdk::ModelName::TROPHY_SYSTEM => &self.config.object,
			sdk::ModelName::ZIPLINE => &self.config.object,
			_ => return,
		};

		highlight(api, ctx, anim.index, anim.entity_ptr, style, &[1.0, 0.5, 0.0]);
	}
}

fn highlight(api: &mut Api, ctx: &mut RunContext, index: u32, entity_ptr: sdk::Ptr, style: &Style, color: &[f32; 3]) {
	// Write the highlight parameters
	let ptr: sdk::Ptr<sdk::HighlightSettings> = entity_ptr.field(ctx.data.entity_highlight);
	if let Ok(mut highlight) = api.vm_read(ptr) {
		if highlight.current_context_id >= 0 && ctx.entity_check(api, index, entity_ptr) {
			if style.enable || highlight.current_context_id == sdk::HIGHLIGHT_CONTEXT_PRIVATE_MATCH_OBSERVER {
				let idx = sdk::HIGHLIGHT_CONTEXT_PRIVATE_MATCH_OBSERVER as usize;
				highlight.current_context_id =
					if style.enable { sdk::HIGHLIGHT_CONTEXT_PRIVATE_MATCH_OBSERVER }
					else if highlight.server_context_id >= 0 { highlight.server_context_id }
					else if highlight.client_context_id >= 0 { highlight.client_context_id }
					else { sdk::HIGHLIGHT_CONTEXT_NEUTRAL };
				highlight.visibility_type = sdk::HIGHLIGHT_VIS_FORCE_ON;
				// highlight.flag |= sdk::HIGHLIGHT_FLAG_CHECK_OFTEN;
				highlight.params[idx].color[0] = color[0] * style.brightness;
				highlight.params[idx].color[1] = color[1] * style.brightness;
				highlight.params[idx].color[2] = color[2] * style.brightness;
				highlight.server_function_bits[idx] = style.to_bits();
				highlight.client_function_bits[idx] = style.to_bits();
				let _ = api.vm_write(ptr, &highlight);
			}
		}
	}
}

// Checks if the object is in FOV of camera player
// This helps reducing glow objects which are not in FOV
fn is_in_fov(camera: &PlayerEntity, origin: &[f32; 3]) -> bool {
	let v = sdk::sub(*origin, camera.origin);

	let len2 = sdk::len2(v);
	if len2 == 0.0 {
		return false;
	}
	if len2 <= 500.0 * 500.0 {
		return true;
	}
	if len2 >= 5000.0 * 5000.0 {
		return false;
	}

	let v = sdk::muls(v, 1.0 / len2.sqrt());
	let view = sdk::qvec(camera.camera_angles);

	sdk::dot(v, view) >= 0.5
}
