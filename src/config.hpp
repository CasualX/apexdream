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
	void reload(WeaponID weapon, CheatManager& cheats) const;

public:
	WeaponID active_id = static_cast<WeaponID>(~0u);
};
