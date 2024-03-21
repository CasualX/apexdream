use bitset_core::BitSet;
use crate::*;

struct Config {
	enable: bool,
}
impl Default for Config {
	fn default() -> Self {
		Config {
			enable: true,
		}
	}
}

#[derive(Default)]
pub struct Observers {
	config: Config,
}


impl Observers {
	pub fn render(&mut self, api: &mut Api, ctx: &RunContext) {
		let state = ctx.state;

		if !state.is_in_game() {
			return;
		}

		let local = state.local_player();
		let fixed_cam = local
			.map(|local| local.observer_mode == sdk::OBS_MODE_FIXED)
			.unwrap_or(false);

		// let mut players = state.players().collect::<Vec<_>>();
		// players.sort_unstable_by_key(|player| (player.team_num, player.team_member_index));

		// if !fixed_cam {
		// 	let mut team_num = -1;
		// 	players.retain(|player| {
		// 		if player.team_num == team_num {
		// 			return false;
		// 		}
		// 	});
		// }

		// Check which teams have anyone alive
		let mut teams = [0u32; 256 / 32];
		for player in state.players() {
			if player.is_alive() {
				let team_index = player.team_num as usize;
				if team_index < teams.bit_len() {
					teams.bit_set(team_index);
				}
			}
		}

		// Compute a bitmap of players observing the local player
		// If at least one person is alive
		let mut mask = [0u32; sdk::MAX_PLAYERS / 32];
		let mut count = 0;
		for player in state.players() {
			if !fixed_cam {
				// Look for all dead players
				if player.is_alive() {
					continue;
				}
				// Their entire team is dead
				let team_index = player.team_num as usize;
				if team_index < teams.bit_len() && teams.bit_test(team_index) {
					continue;
				}
			}
			// Add it to the mask
			let player_index = (player.index as usize).wrapping_sub(1);
			if player_index < sdk::MAX_PLAYERS {
				mask.bit_set(player_index);
				count += 1;
			}
		}

		const POS_X: f32 = 43.0;
		const POS_Y: f32 = 380.0;
		const GUTTER: f32 = 6.0;
		const LINE_HEIGHT: f32 = 12.0;
		const WIDTH: f32 = 140.0;

		api.r_text(
			/*font:*/ 0,
			/*flags:*/ 3,
			/*x:*/ POS_X + GUTTER,
			/*y:*/ POS_Y - 16.0,
			/*width:*/ WIDTH,
			/*height:*/ 100.0,
			/*color:*/ vgc::sRGBA!(White),
			/*shadow:*/ vgc::sRGBA!(Black),
			/*text:*/ s!("Spectators"),
		);

		if count > 0 {
			api.r_rect(POS_X, POS_Y, WIDTH, GUTTER + GUTTER + count as f32 * LINE_HEIGHT, vgc::sRGBA!(Black, 0x40), vgc::sRGBA!(Black, 0xff));
			let mut height = POS_Y + GUTTER;
			s! { let unknown = "Unknown"; }
			for i in 0..sdk::MAX_PLAYERS {
				if mask.bit_test(i) {
					let handle = sdk::EHandle::from(i as u32 + 1);
					let player = match state.entity_as::<PlayerEntity>(handle) {
						Some(player) => player,
						None => continue,
					};
					let name = state.get_player_name(handle).unwrap_or(unknown);
					let same_angles = match local {
						Some(local) =>
							f32::abs(local.server_angles[0] - player.server_angles[0]) < 1.0 &&
							f32::abs(local.server_angles[1] - player.server_angles[1]) < 1.0,
						None => false,
					};
					api.r_text(
						/*font:*/ 0,
						/*flags:*/ 1,
						/*x:*/ POS_X + GUTTER,
						/*y:*/ height,
						/*width:*/ 300.0,
						/*height:*/ 100.0,
						/*color:*/ vgc::sRGB::from(player.team_color).alpha(255),
						/*shadow:*/ vgc::sRGBA!(Black),
						/*text:*/ &fmtools::format!(if same_angles {"!! "} {name}),
					);
					height += LINE_HEIGHT;
				}
			}
		}
	}
}
