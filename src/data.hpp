#pragma once

#include <cstdint>

class GameData {
public:
	GameData() = default;

	uint32_t time_date_stamp = 0x5dfc0d27;
	uint32_t checksum = 0x1f3c00c;

	uint32_t client_state = 0x1126be0;
	uint32_t signon_state = 0x1126c78;
	uint32_t level_name = 0x1126d98;

	uint32_t view_render = 0xcac7638;
	uint32_t view_matrix = 0x1a93d0;
	uint32_t global_vars = 0x11268e0;

	uint32_t input_system = 0x1196540;
	uint32_t input_button_state = 0xb0;

	uint32_t in_attack = 0x0cae9f98;
	uint32_t in_jump = 0x0caea008;

	uint32_t local_entity = 0x104f9e4;
	uint32_t entity_list = 0x1855fd8;

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
	uint32_t player_zoom_state = 0x1880;
	uint32_t player_camera_data = 0x1b68;
	uint32_t player_bleedout_state = 0x2348;
	uint32_t player_observer_mode = 0x2f4c;
	uint32_t player_helmet_armor_type = 0x3e84;
	uint32_t player_latest_primary_weapons = 0x1704;

	uint32_t weaponx_weapon_owner = 0x1300;
	uint32_t weaponx_player_data_zoom_fov = 0x1368 + 0x00b4;
	uint32_t weaponx_weapon_name_index = 0x14ac;
	uint32_t weaponx_projectile_speed = 0x1ad4;

	uint32_t prop_survival = 0x1304;

	uint32_t player_resources = 0xcae8490;
	uint32_t player_resource_names = 0x0a00;

	uint32_t world_death_field = 0x0a40;
};
