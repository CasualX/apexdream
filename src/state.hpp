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
	inline const PlayerEntity* camera_player() const {
		const auto local = local_player();
		if (!local || local->is_alive()) {
			return local;
		}
		return get_entity<PlayerEntity>(local->observer_target);
	}
	inline const WeaponXEntity* active_weapon() const {
		const auto local = local_player();
		if (!local) {
			return nullptr;
		}
		return get_entity<WeaponXEntity>(local->active_weapon());
	}
	inline const WorldEntity* world_entity() const {
		return get_entity<WorldEntity>(EHandle{0});
	}
	template<typename TEntity>
	inline const TEntity* get_entity(EHandle handle) const {
		return handle.is_valid() ? dynamic_cast<const TEntity*>(entities[handle.index()].get()) : nullptr;
	}
	inline WeaponName weapon_name(WeaponIndex weapon_index) const {
		size_t index = static_cast<uint32_t>(weapon_index);
		return static_cast<WeaponName>(index < weapon_names.size() ? hash(weapon_names[index].c_str()) : 0);
	}
	inline const char* get_player_name(EHandle handle) const {
		if (!handle.is_valid()) {
			return nullptr;
		}
		size_t index = handle.index() - 1;
		if (index >= MAX_PLAYERS) {
			return nullptr;
		}
		return player_names[index].c_str();
	}

public:
	SignonState signon_state;
	bool connected;
	char level_buffer[0x40];

	EHandle local_entity;
	EHandle resources_entity;

	uint32_t max_clients;
	float curtime;
	float interval_per_tick;

	float view_matrix[16];
	uint32_t button_state[4];

	std::unique_ptr<std::unique_ptr<BaseEntity>[]> entities;
	std::unique_ptr<CEntInfo[]> ent_info;
	std::unique_ptr<CEntInfo[]> prev_info;

	std::unique_ptr<std::string[]> player_names;
	std::unique_ptr<NameEntry[]> player_ptrs1;
	std::unique_ptr<NameEntry[]> player_ptrs2;

	std::vector<std::string> weapon_names;
};
