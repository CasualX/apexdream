#include "highlight.hpp"
#include "context.hpp"
#include "state.hpp"
#include "process.hpp"
#include "entities.hpp"

#include <cstdint>
#include <cmath>

// Packs red, green, blue color components a 32-bit integer.
constexpr uint32_t RGB(uint8_t r, uint8_t g, uint8_t b) {
	return ((uint32_t)r << 16) | ((uint32_t)g << 8) | (uint32_t)b;
}

// Randomly chosen colors.
// TODO: Match with the team's dive trail color?
const uint32_t TEAM_COLORS[20] = {
	RGB(242, 86, 38),
	RGB(97, 92, 81),
	RGB(174, 247, 89),
	RGB(102, 214, 173),
	RGB(98, 244, 234),
	RGB(92, 208, 250),
	RGB(93, 137, 238),
	RGB(164, 105, 252),
	RGB(243, 98, 161),
	RGB(214, 67, 67),
	RGB(230, 116, 51),
	RGB(185, 179, 167),
	RGB(148, 200, 65),
	RGB(86, 174, 91),
	RGB(55, 188, 200),
	RGB(84, 169, 212),
	RGB(98, 121, 203),
	RGB(102, 61, 174),
	RGB(218, 73, 145),
	RGB(158, 178, 199),
};

void Highlight::run(GameContext& ctx) {
	if (!(ctx.state.is_in_game() && ctx.state.is_any_button_down() && enable)) {
		return;
	}
	const auto local = ctx.state.local_player();
	for (uint32_t i = 1; i <= 64; i += 1) {
		if (const auto player = ctx.state.get_entity<PlayerEntity>(EHandle{i})) {
			// Do not glow team members
			if (local && local->team_num == player->team_num) {
				continue;
			}
			// Brightness and dim if the player is downed
			float mult = brightness / 255.0f;
			if (player->is_downed()) {
				mult *= 0.4f;
			}
			// Grab a nice color per team
			const auto srgb = (player->team_num < 1) ? RGB(255, 255, 255) : TEAM_COLORS[(player->team_num - 1) % 20];
			const float color[3] = {
				((srgb >> 16) & 0xff) * mult,
				((srgb >> 8) & 0xff) * mult,
				((srgb >> 0) & 0xff) * mult,
			};
			static const float TIMES[7] = { INFINITY, INFINITY, INFINITY, INFINITY, INFINITY, INFINITY, INFINITY };
			// Write the highlight params
			if (ctx.entity_check(player->handle, player->address)) {
				ctx.process.write<uint8_t>(player->address + 0x388, 1);
				ctx.process.write<int32_t>(player->address + 0x310, 1);
				ctx.process.write(player->address + 0x1D0, color);
				ctx.process.write(player->address + 0x2D0, TIMES);
			}
		}
	}
}
