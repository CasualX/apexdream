use std::fmt::Write;
use pelite;
use pelite::pe64::*;
use pelite::pattern as pat;

pub fn print(f: &mut super::Output, bin: PeFile<'_>) {
	let _ = writeln!(f.human, "## Miscellaneous\n\n```");
	let _ = writeln!(f.ini, "[Miscellaneous]");
	header(f, bin);
	game_version(f, bin);
	entity_list(f, bin);
	local_entity_handle(f, bin);
	local_player(f, bin);
	global_vars(f, bin);
	input_system(f, bin);
	name_list(f, bin);
	view_render(f, bin);
	client_state(f, bin);
	// unknown_magic(f, bin);
	last_visible(f, bin);
	weapon_projectile(f, bin);
	local_camera(f, bin);
	studio_hdr(f, bin);
	highlight_settings(f, bin);
	network_var(f, bin);
	let _ = writeln!(f.human, "```\n");
	let _ = writeln!(f.ini);
}

fn header(f: &mut super::Output, bin: PeFile<'_>) {
	// Check if offsets are correct
	let time_date_stamp = bin.file_header().TimeDateStamp;
	let check_sum = bin.optional_header().CheckSum;
	let _ = writeln!(f.human, "TimeDateStamp = {:#x}", time_date_stamp);
	let _ = writeln!(f.human, "CheckSum = {:#x}", check_sum);
	let _ = writeln!(f.ini, "TimeDateStamp={:#x}", time_date_stamp);
	let _ = writeln!(f.ini, "CheckSum={:#x}", check_sum);
}

fn entity_list(f: &mut super::Output, bin: PeFile<'_>) {
	// EntityList
	//
	// Find GetEntityByIndex:
	// "Index must be less than %i.\n"
	//
	// entity_ptr = *(uintptr_t*)(entity_list + index * 32)
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("81F9u4 7C% 85C9 79% 4863C1 488D15$'"), &mut save) {
		let num_ent_entries = save[1];
		let cl_entitylist = save[2];
		let _ = writeln!(f.ini, "NUM_ENT_ENTRIES={:#x}", num_ent_entries);
		let _ = writeln!(f.ini, "cl_entitylist={:#x}", cl_entitylist);
	}
	else {
		crate::print_error("unable to find cl_entitylist!");
	}
}

fn local_entity_handle(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];
	// old: pat!("833D${?'}FF 74? 0FB70D${'} 0FB705${'}")
	if bin.scanner().matches_code(pat!("8B05${'} 440FB705???? 83F8FF 74")).next(&mut save) {
		let local_entity_handle = save[1];
		let _ = writeln!(f.ini, "LocalEntityHandle={:#x}", local_entity_handle);
	}
	else {
		crate::print_error("unable to find LocalEntityHandle!");
	}
}

fn local_player(f: &mut super::Output, bin: PeFile<'_>) {
	// The global instance of C_GameMovement contains as its first member a pointer to local player right after its vtable.
	let mut save = [0; 4];
	// old: pat!("8981??0000 488B1D${'}")
	if bin.scanner().finds_code(pat!("488B05${'} 488D0D???? 4488????? 4C89"), &mut save) {
		let local_player_ptr = save[1] + 8;
		let _ = writeln!(f.ini, "LocalPlayer={:#x}", local_player_ptr);
	}
	else {
		crate::print_error("unable to find LocalPlayerPtr!");
	}
}

fn global_vars(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];
	// old: pat!("488B15${*'} BE01000000 8B4234 3BC6")
	// old: pat!("F20F11?${'} [50-100] F30F11????? F30F11????? F30F11????? F30F11????? F30F11????? 72")
	if bin.scanner().finds_code(pat!("488B01 488D15${'} FF5010 85C0 75"), &mut save) {
		let global_vars = save[1];
		let _ = writeln!(f.ini, "GlobalVars={:#x}", global_vars);
	}
	else {
		crate::print_error("unable to find GlobalVars!");
	}
}

fn name_list(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];
	if bin.scanner().matches_code(pat!("48634338 488D0D${'} 4803C0 488B44C1F0")).next(&mut save) {
		let name_list = save[1];
		let _ = writeln!(f.ini, "NameList={:#x}", name_list);
	}
	else {
		crate::print_error("unable to find NameList!");
	}
}

fn game_version(f: &mut super::Output, bin: PeFile<'_>) {
	// References "gameversion.txt"
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("8D?${'\"v\"} [1-32] 8D?$\"gameversion.txt\"00"), &mut save) {
		let game_version = bin.derva_c_str(save[1]).unwrap().to_str().unwrap();
		let _ = writeln!(f.human, "GameVersion = {:?}", game_version);
		let _ = writeln!(f.ini, "GameVersion={}", game_version);
	}
	else {
		crate::print_error("unable to find GameVersion!");
	}
}

fn view_render(f: &mut super::Output, bin: PeFile<'_>) {
	let view_render;
	let view_matrix;

	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("0F 85 ? ? ? ? 48 8B 0D ${'} 48 8B 01 FF 50 20 48 8B 0D"), &mut save) {
		view_render = save[1];
	}
	else {
		crate::print_error("unable to find ViewRender");
		return;
	}

	// old: pat!("480fbec2 488b84c1u4 c3")
	let pattern = pat!("4889[5] 498BB6u4 498B9E[4] E8");
	if bin.scanner().finds_code(pattern, &mut save) {
		view_matrix = save[1];
	}
	else {
		crate::print_error("unable to find ViewMatrix");
		return;
	}

	let _ = writeln!(f.ini, "ViewRender={:#x}", view_render);
	let _ = writeln!(f.ini, "ViewMatrix={:#x}", view_matrix);
}

fn client_state(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];

	// Address of global CClientState instance
	// old: pat!("488D15${\"Missing client class\"} [1-10] 75% 488D0D$'")
	if bin.scanner().finds_code(pat!("488D5424?488B05${'} 488D0D [4] 48897C24?"), &mut save) {
		let client_state = save[1];
		let _ = writeln!(f.ini, "ClientState={:#x}", client_state);
	}
	else {
		crate::print_error("unable to find ClientState");
		return;
	}

	/*
	// SignonState = ClientState + 0x98 has values 0...8
	if bin.scanner().finds_code(pat!("@3 833D${?'}08 0F94C0 C3")) {
		let signon_state = save[1];
		println!("{}!{:#x} SignonState", dll_name, signon_state);
	}
	else {
		crate::print_error("unable to find SignonState");
	}
	*/
	// old: "488D05${\"dedicated\"} C3 833D${?'}02 488D05${00} 7C07 488D05${'} C3"
	if bin.scanner().finds_code(pat!("8B05${'} 83F80875418B05 [4]"), &mut save) {
		let signon_state = save[1];
		let _ = writeln!(f.ini, "SignonState={:#x}", signon_state);
	}
	else {
		crate::print_error("unable to find SignonState");
	}

	// LevelName and SignonState together, look for string "dedicated" the smaller of the two routines
	// LevelName is [u8; 0x40] (buffer of 0x40 bytes inlined in the struct)
	// 48 8D 15 ? ? ? ? 48 8B 45 F8 48 8D 0D ? ? ? ? 
	if bin.scanner().finds_code(pat!("488D15${'} 488B45F8488D0D [4]"), &mut save) {
		let level_name = save[1];
		let _ = writeln!(f.ini, "LevelName={:#x}", level_name);
	}
	else {
		crate::print_error("unable to find LevelName");
	}
}

fn unknown_magic(f: &mut super::Output, bin: PeFile<'_>) {
	// Some unknown magic offsets
	let mut save = [0; 4];
	let mut matches = bin.scanner().matches_code(pat!("488D15${'} [5] E8${} 4885C0 74? 8B?u4"));
	while matches.next(&mut save) {
		if save[2] < 0x100000 {
			let name = bin.derva_c_str(save[1]).ok().and_then(|cstr| cstr.to_str().ok()).unwrap_or("");
			let _ = writeln!(f.ini, "CPlayer!{}={:#x}", name, save[2]);
		}
	}
}

fn local_camera(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("488BF9 0F2E89u4 7A"), &mut save) {
		let camera_origin = save[1];
		let camera_angles = save[1] + 12;
		let _ = writeln!(f.ini, "CPlayer!camera_origin={:#x}", camera_origin);
		let _ = writeln!(f.ini, "CPlayer!camera_angles={:#x}", camera_angles);
	}
	else {
		crate::print_error("unable to find CPlayer camera offsets");
	}
}

fn highlight_settings(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("488B15${'} 4C8D0476"), &mut save) {
		let settings = save[1];
		let _ = writeln!(f.ini, "HighlightSettings={:#x}", settings);
	}
	else {
		crate::print_error("unable to find HighlightSettings offsets");
	}
}

fn last_visible(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("4C8BCE 498B3E F3410F1081 u4 663B774E 0F83"), &mut save) {
		let last_vis = save[1];
		let _ = writeln!(f.ini, "CPlayer!lastVisibleTime={:#x}", last_vis);
		let _ = writeln!(f.ini, "CWeaponX!crosshairTargetTime={:#x}", last_vis + 4);
		let _ = writeln!(f.ini, "CWeaponX!lastCrosshairTargetTime={:#x}", last_vis + 8);
	}
	else {
		crate::print_error("unable to find CPlayer lastVisibleTime offsets");
	}
}

fn weapon_projectile(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("F30F108Bu4 F30F1083[4] 488B05[4] F30F1093u4 "), &mut save) {
		let _ = writeln!(f.ini, "CWeaponX!m_flProjectileSpeed={:#x}", save[1]);
		let _ = writeln!(f.ini, "CWeaponX!m_flProjectileScale={:#x}", save[2]);
	}
	else {
		crate::print_error("unable to find CWeaponX m_flProjectileSpeed offsets");
	}
}

fn studio_hdr(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];
	// First call in a function referencing "Entity has no model"
	// old: pat!("4883B9u4? 488BD9 7536")
	if bin.scanner().finds_code(pat!("488B9Fu4 4885DB 75? 488D0D$\"Entity has no model\""), &mut save) {
		let studio_hdr = save[1];
		let _ = writeln!(f.ini, "CBaseAnimating!m_pStudioHdr={:#x}", studio_hdr);
	}
	else {
		crate::print_error("unable to find studio hdr");
	}
}

fn network_var(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];

	if bin.scanner().finds_code(pat!("486BCA38 488D15${'} 837C110C07 [1-20]$\"Network var\""), &mut save) {
		let network_var_table_ptr = save[1];
		let _ = writeln!(f.ini, "NetworkVarTablePtr={:#x}", network_var_table_ptr);
	}
	else {
		crate::print_error("unable to find NetworkVarTablePtr");
	}

	if bin.scanner().finds_code(pat!("81?u4 73% [1-10]$ \"Network var\""), &mut save) {
		let network_var_table_len = save[1];
		let _ = writeln!(f.ini, "NetworkVarTableLen={}", network_var_table_len);
	}
	else {
		crate::print_error("unable to find NetworkVarTableLen");
	}

	// // This is just cl_entitylist
	// if bin.scanner().matches_code(pat!("@4 8B01 83F8FF 741C 0FB7C8 488D15${'}")).next(&mut save) {
	// 	let network_data_table_ptr = save[1];
	// 	let _ = writeln!(f.ini, "NetworkDataTablePtr={:#x}", network_data_table_ptr);
	// }
	// else {
	// 	crate::print_error("unable to find NetworkDataTablePtr");
	// }
}

fn input_system(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("00000000 894C2420 488D0D$'"), &mut save) {
		let input_system = save[1];
		let _ = writeln!(f.ini, "InputSystem={:#x}", input_system);
	}
	else {
		crate::print_error("unable to find InputSystem");
	}
}
