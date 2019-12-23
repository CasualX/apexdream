#pragma once

#include <cstdint>

class GameData {
public:
	GameData() = default;

	uint32_t time_date_stamp = 0x5de73ec3;
	uint32_t checksum = 0x1f20056;

	uint32_t client_state = 0x1110c10;
	uint32_t signon_state = 0x1110ca8;
	uint32_t level_name = 0x1110dc8;

	uint32_t view_render = 0xcab13f8;
	uint32_t view_matrix = 0x1a93d0;
	uint32_t global_vars = 0x1110910;

	uint32_t input_system = 0x117f680;
	uint32_t input_button_state = 0xb0;

	uint32_t local_entity = 0x1039934;
	uint32_t entity_list = 0x183f118;

	uint32_t entity_model_name = 0x0030;
	uint32_t entity_flags = 0x0098;
	uint32_t entity_origin = 0x014c;
	uint32_t entity_team_num = 0x03f0;
	uint32_t entity_velocity = 0x041c;
	uint32_t entity_bones = 0x0ee0;

	uint32_t player_shields = 0x0170;
	uint32_t player_health = 0x03e0;
	uint32_t player_max_health = 0x0510;
	uint32_t player_life_state = 0x0730;
	uint32_t player_zoom_state = 0x1880;
	uint32_t player_camera_data = 0x1b64;
	uint32_t player_bleedout_state = 0x2348;
	uint32_t player_observer_mode = 0x2f3c;
	uint32_t player_helmet_armor_type = 0x3e64;
	uint32_t player_latest_primary_weapons = 0x1704;

	uint32_t weaponx_weapon_owner = 0x1300;
	uint32_t weaponx_player_data_zoom_fov = 0x1368 + 0x00b0;
	uint32_t weaponx_weapon_name_index = 0x14a8;
	uint32_t weaponx_projectile_speed = 0x1ad0;

	uint32_t prop_survival = 0x1304;

	uint32_t player_resources = 0xcad2248;
	uint32_t player_resource_names = 0x0a00;

	uint32_t world_death_field = 0x0a40;
};
