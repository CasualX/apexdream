#pragma once

#include <cstdint>

class GameData {
public:
	GameData() = default;

	uint32_t time_date_stamp = 0x5e31fce3;
	uint32_t checksum = 0x1f460ef;

	uint32_t client_state = 0x112eac0;
	uint32_t client_signon_state = 0x0098;
	uint32_t client_level_name = 0x01b8;

	uint32_t view_render = 0xcacf788;
	uint32_t view_matrix = 0x1a93d0;
	uint32_t global_vars = 0x112e7c0;

	uint32_t input_system = 0x119e4c0;
	uint32_t input_button_state = 0xb0;

	uint32_t in_attack = 0x0caf20e8;
	uint32_t in_jump = 0x0caf2160;

	uint32_t local_entity = 0x1057a84;
	uint32_t entity_list = 0x185db98;

	uint32_t entity_model_name = 0x0030;
	uint32_t entity_flags = 0x0098;
	uint32_t entity_origin = 0x014c;
	uint32_t entity_team_num = 0x03f0;
	uint32_t entity_velocity = 0x041c;
	uint32_t entity_bones = 0x0ee0;

	uint32_t highlight_enable = 0x388;
	uint32_t highlight_index = 0x310;
	uint32_t highlight_color = 0x1d0;
	uint32_t highlight_fade = 0x2d0;

	uint32_t player_shields = 0x0170;
	uint32_t player_health = 0x03e0;
	uint32_t player_max_health = 0x0510;
	uint32_t player_life_state = 0x0730;
	uint32_t player_latest_primary_weapons = 0x1944;
	uint32_t player_zoom_state = 0x1ac0;
	uint32_t player_camera_data = 0x1da8;
	uint32_t player_bleedout_state = 0x2588;
	uint32_t player_observer_mode = 0x31cc;
	uint32_t player_helmet_armor_type = 0x4104;

	uint32_t weaponx_weapon_owner = 0x1540;
	uint32_t weaponx_player_data_zoom_fov = 0x165c;
	uint32_t weaponx_weapon_name_index = 0x16ec;
	uint32_t weaponx_projectile_speed = 0x1d14;

	uint32_t prop_survival = 0x1544;

	uint32_t player_resources = 0xcaf05e0;
	uint32_t player_resource_names = 0x0a00;

	uint32_t world_death_field = 0x0a40;
};
