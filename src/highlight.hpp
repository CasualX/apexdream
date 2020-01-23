#pragma once

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
public:
	HighlightConfig config;
};
