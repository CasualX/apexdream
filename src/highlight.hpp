#pragma once

class GameContext;

class Highlight {
public:
	Highlight() = default;
	void run(GameContext& ctx);
private:
	// Configuration.
	bool enable = true;
	// Brightness factor of the highlight outlines effect.
	float brightness = 2.55f;
};
