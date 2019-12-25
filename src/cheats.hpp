#pragma once

#include "highlight.hpp"
#include "aimassist.hpp"

class GameContext;

// Owns and invokes all the cheat modules.
class CheatManager {
public:
	CheatManager() = default;
	void run(GameContext& ctx);
private:
	Highlight highlight;
	AimAssist aimassist;
};
