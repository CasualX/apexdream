#pragma once

#include "highlight.hpp"
#include "aimassist.hpp"
#include "scripts.hpp"

class GameContext;

// Owns and invokes all the cheat modules.
class CheatManager {
public:
	CheatManager() = default;
	void run(GameContext& ctx);
private:
	Highlight highlight;
	AimAssist aimassist;
	Scripts scripts;
};
