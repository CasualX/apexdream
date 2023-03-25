
#[derive(Default)]
pub struct GameData {
	pub time_date_stamp: u32,
	pub checksum: u32,

	pub global_vars: u32,

	pub entity_list: u32,
	pub local_entity_handle: u32,

	pub input: u32,
	pub input_selected_slot: u32,

	pub client_state: u32,
	pub client_signon_state: u32,
	pub client_level_name: u32,

	pub nst_weapon_names: u32,

	pub view_render: u32,
	pub view_matrix: u32,

	pub input_system: u32,
	pub input_button_state: u32,

	pub name_list: u32,

	pub network_var_table_ptr: u32,
	pub network_var_table_len: u32,

	pub thirdperson_override: u32,
	pub mouse_sensitivity: u32,
	pub fps_max: u32,
	pub mp_gamemode: u32,

	pub in_attack: u32,
	pub in_jump: u32,
	pub in_duck: u32,
	pub in_reload: u32,
	pub in_use: u32,
	pub in_zoom: u32,
	pub in_forward: u32,
	pub in_backward: u32,
	pub in_moveleft: u32,
	pub in_moveright: u32,

	pub entity_model_name: u32,
	pub entity_view_offset: u32,
	pub entity_flags: u32,
	pub entity_origin: u32,
	pub entity_shield_health: u32,
	pub entity_highlight: u32,
	pub entity_health: u32,
	pub entity_team_num: u32,
	pub entity_velocity: u32,
	pub entity_owner_entity: u32,
	pub entity_max_health: u32,
	pub entity_life_state: u32,

	pub animating_skin: u32,
	pub animating_bone_array: u32, // m_bSequenceFinished - 0x1C
	pub animating_studiohdr: u32, // m_flModelScale + 0x1D0

	pub bcc_next_attack: u32,
	pub bcc_inventory: u32,
	pub bcc_selected_weapons: u32,
	pub bcc_last_visible_time: u32, // m_hudInfo_visibilityTestAlwaysPasses + 0x3

	pub player_zoom_state: u32,
	pub player_camera_data: u32,
	pub player_time_base: u32,
	pub player_server_angles: u32,
	pub player_view_angles: u32,
	pub player_weapon_punch: u32,
	pub player_consumables: u32,
	pub player_platform_uid: u32,
	pub player_bleedout_state: u32,
	pub player_movement_state: u32,
	pub player_observer_state: u32,
	pub player_third_person_shoulder_view: u32,
	pub player_script_net_data: u32,
	pub player_helmet_armor_type: u32,

	pub weaponx_weapon_owner: u32,
	pub weaponx_next_primary_attack: u32,
	pub weaponx_ammo_in_clip: u32,
	pub weaponx_player_data: u32,
	pub weaponx_zoom_fov: u32,
	pub weaponx_mod_bitfield: u32,
	pub weaponx_weapon_name_index: u32,
	pub weaponx_is_semi_auto: u32,
	pub weaponx_projectile_speed: u32,
	pub weaponx_charge_start_time: u32,
	pub weaponx_burst_fire: u32,

	pub vehicle_driver: u32,
	pub vehicle_velocity: u32,

	pub prop_survival: u32,
	pub projectile: u32,
	pub world_death_field: u32,
	pub waypoint_type: u32,

	pub mods_names: u32,
	pub mods_list: u32,
	pub mods_count: u32,
}

impl GameData {
	pub fn load(&mut self, gd: &str, tds: u32) -> bool {
		use std::collections::HashMap;
		use ini_core::*;
		let mut map = HashMap::new();
		macro_rules! build_map {
			($($field:ident,)*) => {
				$(
					let $field = hash!(stringify!($field));
					map.insert($field, &mut self.$field);
				)*
			};
		}
		build_map! {
			time_date_stamp,
			checksum,
			global_vars,
			entity_list,
			local_entity_handle,
			input,
			input_selected_slot,
			client_state,
			client_signon_state,
			client_level_name,
			nst_weapon_names,
			view_render,
			view_matrix,
			input_system,
			input_button_state,
			name_list,
			network_var_table_ptr,
			network_var_table_len,
			thirdperson_override,
			mouse_sensitivity,
			fps_max,
			mp_gamemode,
			in_attack,
			in_jump,
			in_duck,
			in_reload,
			in_use,
			in_zoom,
			in_forward,
			in_backward,
			in_moveleft,
			in_moveright,
			entity_model_name,
			entity_view_offset,
			entity_flags,
			entity_origin,
			entity_shield_health,
			entity_highlight,
			entity_health,
			entity_team_num,
			entity_velocity,
			entity_owner_entity,
			entity_max_health,
			entity_life_state,
			animating_skin,
			animating_bone_array,
			animating_studiohdr,
			bcc_next_attack,
			bcc_inventory,
			bcc_selected_weapons,
			bcc_last_visible_time,
			player_zoom_state,
			player_camera_data,
			player_time_base,
			player_server_angles,
			player_view_angles,
			player_weapon_punch,
			player_consumables,
			player_platform_uid,
			player_bleedout_state,
			player_movement_state,
			player_observer_state,
			player_third_person_shoulder_view,
			player_script_net_data,
			player_helmet_armor_type,
			weaponx_weapon_owner,
			weaponx_next_primary_attack,
			weaponx_ammo_in_clip,
			weaponx_player_data,
			weaponx_zoom_fov,
			weaponx_mod_bitfield,
			weaponx_weapon_name_index,
			weaponx_is_semi_auto,
			weaponx_projectile_speed,
			weaponx_charge_start_time,
			weaponx_burst_fire,
			vehicle_driver,
			vehicle_velocity,
			prop_survival,
			projectile,
			world_death_field,
			waypoint_type,
			mods_names,
			mods_list,
			mods_count,
		}

		for line in Parser::new(gd) {
			match line {
				Item::Section(_) => {
					// Stop loading data when we found a section with matching offsets
					if map.get(&time_date_stamp).map(|&&mut v| v) == Some(tds) {
						return true;
					}
				},
				Item::Property(key, value) => {
					let key = crate::hash(key);
					if let Some(key) = map.get_mut(&key) {
						use crate::base::parse_u32;
						**key = parse_u32(value);
					}
				},
				_ => (),
			}
		}

		// Stop loading data when we found a section with matching offsets
		map.get(&time_date_stamp).map(|&&mut v| v) == Some(tds)
	}
}
