use std::fs;
use std::collections::HashMap;

fn main() {
	let offsets = fs::read_to_string("offsets/stdout.ini").unwrap();

	let map = parse(&offsets);

	println!("[Offsets]");
	if let Some(&game_version) = map.get(&("Miscellaneous", "GameVersion")) {
		println!("; {}", game_version);
	}

	print(&map, "time_date_stamp", "Miscellaneous", "TimeDateStamp");
	print(&map, "checksum", "Miscellaneous", "CheckSum");
	print(&map, "global_vars", "Miscellaneous", "GlobalVars");
	print(&map, "entity_list", "Miscellaneous", "cl_entitylist");
	print(&map, "local_entity_handle", "Miscellaneous", "LocalEntityHandle");
	print2(&map, "input", "Globals", ".?AVCInput@@", 0);
	println!("input_selected_slot=0x180");
	print(&map, "client_state", "Miscellaneous", "ClientState");
	println!("client_signon_state=0xAC");
	println!("client_level_name=0x1C0");
	print2(&map, "nst_weapon_names", "NetworkedStringTables", "WeaponNames", 0);
	print(&map, "view_render", "Miscellaneous", "ViewRender");
	print(&map, "view_matrix", "Miscellaneous", "ViewMatrix");
	print(&map, "input_system", "Miscellaneous", "InputSystem");
	println!("input_button_state=0xB0");
	print(&map, "name_list", "Miscellaneous", "NameList");
	print(&map, "network_var_table_ptr", "Miscellaneous", "NetworkVarTablePtr");
	print(&map, "network_var_table_len", "Miscellaneous", "NetworkVarTableLen");

	print2(&map, "thirdperson_override", "ConVars", "thirdperson_override", 0);
	print2(&map, "mouse_sensitivity",    "ConVars", "mouse_sensitivity", 0);
	print2(&map, "fps_max",              "ConVars", "fps_max", 0);
	print2(&map, "mp_gamemode",          "ConVars", "mp_gamemode", 0);

	print2(&map, "in_attack",    "Buttons", "in_attack", 0);
	print2(&map, "in_jump",      "Buttons", "in_jump", 0);
	print2(&map, "in_duck",      "Buttons", "in_duck", 0);
	print2(&map, "in_reload",    "Buttons", "in_reload", 0);
	print2(&map, "in_use",       "Buttons", "in_use", 0);
	print2(&map, "in_zoom",      "Buttons", "in_zoom", 0);
	print2(&map, "in_forward",   "Buttons", "in_forward", 0);
	print2(&map, "in_backward",  "Buttons", "in_backward", 0);
	print2(&map, "in_moveleft",  "Buttons", "in_moveleft", 0);
	print2(&map, "in_moveright", "Buttons", "in_moveright", 0);

	print2(&map, "entity_model_name",    "DataMap.C_BaseEntity", "m_ModelName", 0);
	print2(&map, "entity_view_offset",   "DataMap.C_BaseEntity", "m_currentFrame.viewOffset", 0);
	print2(&map, "entity_flags",         "DataMap.C_Player", "m_fFlags", 0);
	print2(&map, "entity_origin",        "DataMap.C_BaseEntity", "m_vecAbsOrigin", 0);
	print2(&map, "entity_shield_health", "RecvTable.DT_BaseEntity", "m_shieldHealth", 0);
	print2(&map, "entity_highlight",     "RecvTable.DT_HighlightSettings", "m_highlightParams", 0);
	print2(&map, "entity_health",        "RecvTable.DT_Player", "m_iHealth", 0);
	print2(&map, "entity_team_num",      "RecvTable.DT_BaseEntity", "m_iTeamNum", 0);
	print2(&map, "entity_velocity",      "DataMap.C_BaseEntity", "m_vecVelocity", 0);
	print2(&map, "entity_owner_entity",  "RecvTable.DT_BaseEntity", "m_hOwnerEntity", 0);
	print2(&map, "entity_max_health",    "RecvTable.DT_Player", "m_iMaxHealth", 0);
	print2(&map, "entity_life_state",    "RecvTable.DT_Player", "m_lifeState", 0);

	print2(&map, "animating_skin",        "RecvTable.DT_BaseAnimating", "m_nSkin", 0);
	print2(&map, "animating_bone_array",  "RecvTable.DT_BaseAnimating", "m_bSequenceFinished", -0x1C);
	print2(&map, "animating_studiohdr",   "Miscellaneous", "CBaseAnimating!m_pStudioHdr", 0);

	print2(&map, "bcc_next_attack",        "DataMap.C_BaseCombatCharacter", "m_flNextAttack", 0);
	print2(&map, "bcc_inventory",          "RecvTable.DT_Player", "m_inventory", 0);
	print2(&map, "bcc_selected_weapons",   "RecvTable.DT_BaseCombatCharacter", "m_selectedWeapons", 0);
	print2(&map, "bcc_last_visible_time",  "RecvTable.DT_BaseCombatCharacter", "m_hudInfo_visibilityTestAlwaysPasses", 4);

	print2(&map, "player_zoom_state",        "RecvTable.DT_Player", "m_bZooming", 0);
	print2(&map, "player_camera_data",       "Miscellaneous", "CPlayer!camera_origin", 0);
	print2(&map, "player_time_base",         "DataMap.C_Player", "m_currentFramePlayer.timeBase", 0);
	print2(&map, "player_server_angles",     "DataMap.C_Player", "m_currentFramePlayer.m_ziprailBankTiltFrac", 0x4);
	print2(&map, "player_weapon_punch",      "DataMap.C_Player", "m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle", 0);
	print2(&map, "player_view_angles",       "RecvTable.DT_Player", "m_ammoPoolCapacity", -0x14);
	print2(&map, "player_consumables",       "RecvTable.DT_LocalPlayerExclusive", "m_consumableInventory", 0);
	print2(&map, "player_platform_uid",      "RecvTable.DT_Player", "m_platformUserId", 0);
	print2(&map, "player_bleedout_state",    "RecvTable.DT_Player", "m_bleedoutState", 0);
	print2(&map, "player_movement_state",    "RecvTable.DT_Player", "m_duckState", 0);
	print2(&map, "player_observer_state",    "RecvTable.DT_LocalPlayerExclusive", "m_iObserverMode", 0);
	print2(&map, "player_third_person_shoulder_view", "RecvTable.DT_LocalPlayerExclusive", "m_thirdPersonShoulderView", 0);
	print2(&map, "player_script_net_data",   "RecvTable.DT_Player", "m_playerScriptNetDataGlobal", 0);
	print2(&map, "player_helmet_armor_type", "RecvTable.DT_Player", "m_helmetType", 0);

	print2(&map, "weaponx_weapon_owner",        "RecvTable.DT_WeaponX", "m_weaponOwner", 0);
	print2(&map, "weaponx_next_primary_attack", "RecvTable.DT_WeaponX_LocalWeaponData", "m_nextPrimaryAttackTime", 0);
	print2(&map, "weaponx_ammo_in_clip",        "RecvTable.DT_WeaponX_LocalWeaponData", "m_ammoInClip", 0);
	print2(&map, "weaponx_player_data",         "RecvTable.DT_WeaponX", "m_playerData", 0);
	print2(&map, "weaponx_zoom_fov",            "RecvTable.DT_WeaponPlayerData", "m_curZoomFOV", 0);
	print2(&map, "weaponx_charge_start_time",   "RecvTable.DT_WeaponX", "m_chargeStartTime", 0);
	print2(&map, "weaponx_burst_fire",          "DataMap.CWeaponX", "m_burstFireCount", 0);
	print2(&map, "weaponx_mod_bitfield",        "RecvTable.DT_WeaponX", "m_modBitfieldFromPlayer", 0);
	print2(&map, "weaponx_weapon_name_index",   "RecvTable.DT_WeaponX", "m_weaponNameIndex", 0);

	let wsbase = get_int(&map, "WeaponSettingsMeta", "base");
	print2(&map, "weaponx_is_semi_auto",        "WeaponSettings", "is_semi_auto", wsbase);
	print2(&map, "weaponx_projectile_speed",    "WeaponSettings", "projectile_launch_speed", wsbase);

	print2(&map, "vehicle_driver",    "RecvTable.DT_PlayerVehicle", "m_vehicleDriver", 0);
	print2(&map, "vehicle_velocity",  "RecvTable.DT_PlayerVehicle", "m_vehicleVelocity", 0);

	print2(&map, "prop_survival",     "RecvTable.DT_PropSurvival", "m_ammoInClip", 0);
	print2(&map, "projectile",        "RecvTable.DT_Projectile", "m_weaponDataIsSet", 0);
	print2(&map, "world_death_field", "RecvTable.DT_World", "m_deathFieldIsActive", 0);
	print2(&map, "waypoint_type",     "RecvTable.DT_PlayerWaypoint", "m_waypointType", 0);

	print2(&map, "mods_names", "ModifierOffsets", "mods_names", 0);
	print2(&map, "mods_list", "ModifierOffsets", "mods_list", 0);
	print2(&map, "mods_count", "ModifierOffsets", "mods_count", 0);
}

fn parse(s: &str) -> HashMap<(&str, &str), &str> {
	use ini_core::*;
	let mut map = HashMap::new();
	let mut sect = "";
	for line in Parser::new(s) {
		match line {
			Item::Section(section) => {
				sect = section;
			},
			Item::Property(key, value) => {
				let _ = map.insert((sect, key), value);
			},
			_ => (),
		}
	}
	return map;
}

fn get_int(map: &HashMap<(&str, &str), &str>, sect: &str, key: &str) -> i32 {
	let mut value = match map.get(&(sect, key)) {
		Some(&value) => value,
		None => {
			eprintln!("Missing [{}].{}", sect, key);
			return 0;
		}
	};
	let radix;
	if value.starts_with("0x") {
		value = &value[2..];
		radix = 16;
	}
	else {
		radix = 10;
	}
	u32::from_str_radix(value, radix).unwrap() as i32
}

#[inline(never)]
fn print(map: &HashMap<(&str, &str), &str>, name: &str, sect: &str, key: &str) {
	let value = match map.get(&(sect, key)) {
		Some(&value) => value,
		None => {
			eprintln!("Missing [{}].{}", sect, name);
			return;
		}
	};
	println!("{}={}", name, value);
}

#[inline(never)]
fn print2(map: &HashMap<(&str, &str), &str>, name: &str, sect: &str, key: &str, offset: i32) {
	let value = match map.get(&(sect, key)) {
		Some(&value) => value,
		None => {
			eprintln!("Missing [{}].{}", sect, name);
			return;
		}
	};
	let value = if value.starts_with("0x") { u32::from_str_radix(&value[2..], 16) } else { u32::from_str_radix(value, 10) }.unwrap();
	println!("{}={:#x}", name, value.wrapping_add(offset as u32));
}
