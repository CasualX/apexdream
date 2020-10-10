#pragma once

#include "sdk.hpp"
#include "items.hpp"

class CheatManager;
class GameState;

class Config {
public:
	Config() = default;
	void run(const GameState& state, CheatManager& cheats);

	// Load the configuration for the specific weapon
	void reload(WeaponName weapon_name, CheatManager& cheats) const;

public:
	WeaponIndex active_id = static_cast<WeaponIndex>(~0u);
};
