#pragma once

#include "entities.hpp"
#include "sdk.hpp"

#include <memory>

class GameProcess;
class GameData;

class GameState {
public:
	GameState();

	void update(const GameProcess& process, const GameData& data);

	std::unique_ptr<BaseEntity> create_entity(const GameProcess& process, uint32_t index, uint64_t entity_ptr);

	bool is_in_game() const {
		return signon_state == SignonState::Full && level_buffer[0] != '\0' && !!strcmp(level_buffer, "mp_lobby");
	}
	const char* level_name() const {
		return &level_buffer[0];
	}
	bool is_button_down(ButtonCode button_code) const {
		return (button_state[static_cast<uint32_t>(button_code) >> 5] & (1 << (static_cast<uint32_t>(button_code) & 0x1f))) != 0;
	}
	bool is_any_button_down() const {
		return !(button_state[0] == 0 && button_state[1] == 0 && button_state[2] == 0 && button_state[3] == 0);
	}
	const PlayerEntity* local_player() const {
		return get_entity<PlayerEntity>(local_entity);
	}
	const WorldEntity* world_entity() const {
		return get_entity<WorldEntity>(EHandle{0});
	}
	const PlayerResourceEntity* player_resources() const {
		return get_entity<PlayerResourceEntity>(resources_entity);
	}
	template<typename TEntity>
	const TEntity* get_entity(EHandle handle) const {
		return handle.is_valid() ? dynamic_cast<const TEntity*>(entities[handle.index()].get()) : nullptr;
	}

public:
	SignonState signon_state;
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
};
