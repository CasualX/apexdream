#include "cheats.hpp"

void CheatManager::run(GameContext& ctx) {
	highlight.run(ctx);
	aimassist.run(ctx);
}
