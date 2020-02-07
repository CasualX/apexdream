#pragma once

#include "entities.hpp"

class GameContext;

struct HighlightConfig {
	// Configuration.
	bool enable = true;
	// Brightness factor of the highlight outlines effect.
	float brightness = 2.55f;
};

class Highlight {
public:
	Highlight() = default;
	void run(GameContext& ctx);

	void highlight_player(GameContext& ctx, const PlayerEntity* player, const PlayerEntity* local) const;
public:
	HighlightConfig config;
};
