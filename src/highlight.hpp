#pragma once

#include "entities.hpp"
#include "sdk.hpp"
#include "items.hpp"

class GameContext;
class GameState;

struct HighlightConfig {
	// Configuration.
	bool enable = true;
	// Max distance for highlight
	float distance = 5000.0;
	// Brightness factor of the highlight outlines effect.
	float brightness = 2.55f;
	// Dim brightness factor if the target is downed.
	float downed_factor = 0.4f;
	// Dim brightness factor if the target is not visible.
	float hidden_factor = 0.2f;
};

class Highlight {
public:
	Highlight() = default;
	void run(GameContext& ctx);

	void highlight_player(GameContext& ctx, const PlayerEntity* player, const PlayerEntity* camera) const;
	void highlight_loot(GameContext& ctx, const PropSurvivalEntity* loot, const PlayerEntity* camera, const ItemSet& filter) const;

	void update_filter(const GameState& state, const PlayerEntity* camera, ItemSet& filter);

	static void player_items(const PlayerEntity* player, ItemSet& filter);
	static void weapon_attachments(WeaponName weapon_name, ItemSet& filter);
public:
	HighlightConfig config;
	// Filter highlighted loot based on this weapon entity
	EHandle weapon_handle{};
};
