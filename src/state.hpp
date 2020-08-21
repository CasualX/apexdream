#pragma once

#include "entities.hpp"
#include "sdk.hpp"
#include "hash.hpp"

#include <cstring>
#include <memory>
#include <vector>

class GameProcess;

class GameState {
public:
	GameState();

	void update(const GameProcess& process);

	std::unique_ptr<BaseEntity> create_entity(const GameProcess& process, uint32_t index, uint64_t entity_ptr);

	inline bool is_in_game() const {
		return signon_state == SignonState::Full && level_buffer[0] != '\0' && !!strcmp(level_buffer, "mp_lobby");
	}
	inline const char* level_name() const {
		return &level_buffer[0];
	}
	inline bool is_button_down(ButtonCode button_code) const {
		return (button_state[static_cast<uint32_t>(button_code) >> 5] & (1 << (static_cast<uint32_t>(button_code) & 0x1f))) != 0;
	}
	inline bool is_any_button_down() const {
		return !(button_state[0] == 0 && button_state[1] == 0 && button_state[2] == 0 && button_state[3] == 0);
	}
	inline const PlayerEntity* local_player() const {
		return get_entity<PlayerEntity>(local_entity);
	}
	inline const WorldEntity* world_entity() const {
		return get_entity<WorldEntity>(EHandle{0});
	}
	inline const PlayerResourceEntity* player_resources() const {
		return get_entity<PlayerResourceEntity>(resources_entity);
	}
	template<typename TEntity>
	inline const TEntity* get_entity(EHandle handle) const {
		return handle.is_valid() ? dynamic_cast<const TEntity*>(entities[handle.index()].get()) : nullptr;
	}
	inline uint32_t weapon_name(uint32_t weapon_name_index) const {
		return weapon_name_index < weapon_names.size() ? hash(weapon_names[weapon_name_index].c_str()) : 0;
	}

public:
	SignonState signon_state;
	bool connected;
	char level_buffer[0x40];
	EHandle local_entity;
	EHandle resources_entity;
	uint32_t max_clients;
	float curtime;
	float view_matrix[16];
	uint32_t button_state[4];
	std::unique_ptr<std::unique_ptr<BaseEntity>[]> entities;
	std::unique_ptr<CEntInfo[]> ent_info;
	std::unique_ptr<CEntInfo[]> prev_info;
	std::vector<std::string> weapon_names;
};
