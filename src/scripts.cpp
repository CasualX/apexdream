#include "scripts.hpp"
#include "context.hpp"
#include "state.hpp"
#include "entities.hpp"

void Scripts::run(GameContext& ctx) {
	const auto local = ctx.state.local_player();
	if (bunnyhop) {
		ctx.jump.force = true;
		// Very simply bunnyhop:
		// Force the jump button to be released while airborne
		if (local && !local->is_onground()) {
			ctx.jump.release = true;
		}
	}
}
