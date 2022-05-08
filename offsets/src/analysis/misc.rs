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
	name_list(f, bin);
	view_render(f, bin);
	client_state(f, bin);
	projectile_speed(f, bin);
	unknown_magic(f, bin);
	local_camera(f, bin);
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
		crate::print_error(&"unable to find cl_entitylist!");
	}
}

fn local_entity_handle(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];
	if bin.scanner().matches_code(pat!("833D${?'}FF 74? 0FB70D${'} 0FB705${'}")).next(&mut save) {
		let local_entity_handle = save[1];
		let _ = writeln!(f.ini, "LocalEntityHandle={:#x}", local_entity_handle);
	}
	else {
		crate::print_error(&"unable to find LocalEntityHandle!");
	}
}

fn local_player(f: &mut super::Output, bin: PeFile<'_>) {
	// The global instance of C_GameMovement contains as its first member a pointer to local player right after its vtable.
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("48 89 05 ${[8]'} 48 85 C9 74 % 48 8D 05"), &mut save) {
		let local_player_ptr = save[1];
		let _ = writeln!(f.ini, "LocalPlayer={:#x}", local_player_ptr);
	}
	else {
		crate::print_error(&"unable to find LocalPlayerPtr!");
	}
}

fn global_vars(f: &mut super::Output, bin: PeFile<'_>) {
	// Right above "Client.dll Init_PostVideo() in library "
	// lea r8, qword_XXX
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("488B01 488D15${'} [10-20] $\"Client.dll Init_PostVideo\""), &mut save) {
		let global_vars = save[1];
		let _ = writeln!(f.ini, "GlobalVars={:#x}", global_vars);
	}
	else {
		crate::print_error(&"unable to find GlobalVars!");
	}
}

fn name_list(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];
	if bin.scanner().matches_code(pat!("48634338 488D0D${'} 4803C0 488B44C1F0")).next(&mut save) {
		let name_list = save[1];
		let _ = writeln!(f.ini, "NameList={:#x}", name_list);
	}
	else {
		crate::print_error(&"unable to find NameList!");
	}
}

fn game_version(f: &mut super::Output, bin: PeFile<'_>) {
	// References "gameversion.txt"
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("488D1D${'} C605????01 488BD3 488D0D$\"gameversion.txt\"00"), &mut save) {
		let game_version = bin.derva_c_str(save[1]).unwrap().to_str().unwrap();
		let _ = writeln!(f.human, "GameVersion = {:?}", game_version);
		let _ = writeln!(f.ini, "GameVersion={}", game_version);
	}
	else {
		crate::print_error(&"unable to find GameVersion!");
	}
}

fn view_render(f: &mut super::Output, bin: PeFile<'_>) {
	let view_render;
	let view_matrix;

	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!(/*"74 34 48 8B 0D ${'} 40 0F B6 D7"*/ "4C 8B 05 ${'} 48 8D 15 $ [24]* 48 C7"), &mut save) {
		view_render = save[1];
	}
	else {
		crate::print_error(&"unable to find ViewRender");
		return;
	}

	if bin.scanner().finds_code(pat!("480fbec2 488b84c1u4 c3"), &mut save) {
		view_matrix = save[1];
	}
	else {
		crate::print_error(&"unable to find ViewMatrix");
		return;
	}

	let _ = writeln!(f.ini, "ViewRender={:#x}", view_render);
	let _ = writeln!(f.ini, "ViewMatrix={:#x}", view_matrix);
}

fn client_state(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 4];

	// Address of global CClientState instance
	if bin.scanner().finds_code(pat!("488D15${\"Missing client class\"} (???75%|) 488D0D$'"), &mut save) {
		let client_state = save[1];
		let _ = writeln!(f.ini, "ClientState={:#x}", client_state);
	}
	else {
		crate::print_error(&"unable to find ClientState");
		return;
	}


	/*
	// SignonState = ClientState + 0x98 has values 0...8
	if bin.scanner().finds_code(pat!("@3 833D${?'}08 0F94C0 C3")) {
		let signon_state = save[1];
		println!("{}!{:#x} SignonState", dll_name, signon_state);
	}
	else {
		crate::print_error(&"unable to find SignonState");
	}
	*/
	// LevelName and SignonState together, look for string "dedicated" the smaller of the two routines
	// LevelName is [u8; 0x40] (buffer of 0x40 bytes inlined in the struct)
	if bin.scanner().finds_code(pat!("488D05${\"dedicated\"} C3 833D${?'}02 488D05${00} 7C07 488D05${'} C3"), &mut save) {
		let signon_state = save[1];
		let level_name = save[2];
		let _ = writeln!(f.ini, "SignonState={:#x}", signon_state);
		let _ = writeln!(f.ini, "LevelName={:#x}", level_name);
	}
	else {
		crate::print_error(&"unable to find LevelName");
	}
}

fn projectile_speed(f: &mut super::Output, bin: PeFile<'_>) {
	// Find near the string 'Speed(%f) is greater than sv_maxvelocity(%f)'
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("488B05${*[24]*\"sv_maxvelocity\"} F30F59?u4"), &mut save) {
		let projectile_speed = save[1];
		let projectile_scale = save[1] + 8;
		let _ = writeln!(f.ini, "CWeaponX!m_flProjectileSpeed={:#x}", projectile_speed);
		let _ = writeln!(f.ini, "CWeaponX!m_flProjectileScale={:#x}", projectile_scale);
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
	if bin.scanner().finds_code(pat!("f3 0f 10 83 u4 f3 0f 10 8b d4 1e 00 00"), &mut save) {
		let camera_origin = save[1];
		let camera_angles = save[1] + 12;
		let _ = writeln!(f.ini, "CPlayer!camera_origin={:#x}", camera_origin);
		let _ = writeln!(f.ini, "CPlayer!camera_angles={:#x}", camera_angles);
	}
}
