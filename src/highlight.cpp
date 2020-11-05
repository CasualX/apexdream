#include "highlight.hpp"
#include "context.hpp"
#include "state.hpp"
#include "process.hpp"
#include "data.hpp"
#include "sdk.hpp"

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
	if (!config.enable || !ctx.state.is_in_game()) {
		return;
	}
	const auto camera = ctx.state.camera_player();
	if (!camera) {
		return;
	}
	// Here's the deal: writing highlight parameters is risky
	// Due to the racy nature of writing the highlight parameters
	// Reduce to chance of crashes by only writing if any key is being pressed
	if (!(ctx.state.is_any_button_down() && camera->origin.z < 11000.0)) {
		return;
	}
	// Update the loot filter
	ItemSet filter{};
	update_filter(ctx.state, camera, filter);
	for (uint32_t i = 1; i <= NUM_ENT_ENTRIES; i += 1) {
		const auto handle = EHandle{i};
		if (const auto player = ctx.state.get_entity<PlayerEntity>(handle)) {
			highlight_player(ctx, player, camera);
		}
		// Disabled for now as it's just too slow...
		// else if (const auto loot = ctx.state.get_entity<PropSurvivalEntity>(handle)) {
		// 	highlight_loot(ctx, loot, camera, filter);
		// }
	}
}
void Highlight::highlight_player(GameContext& ctx, const PlayerEntity* player, const PlayerEntity* camera) const {
	// Do not glow team members
	if (camera->team_num == player->team_num) {
		return;
	}
	// Brightness and dim if the player is downed
	float mult = config.brightness / 255.0f;
	if (player->is_downed()) {
		mult *= config.downed_factor;
	}
	if (!player->is_visible(ctx.state.curtime)) {
		mult *= config.hidden_factor;
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
	if (ctx.entity_check(player)) {
		ctx.process.write<bool>(player->address + 0x3E0, true);
		ctx.process.write<int32_t>(player->address + 0x350, 1);
		ctx.process.write(player->address + 0x1B8 + 12 * 2, color);
		ctx.process.write(player->address + 0x310, TIMES);
		ctx.process.write(player->address + 0x33C, config.distance);
	}
}
void Highlight::highlight_loot(GameContext& ctx, const PropSurvivalEntity* loot, const PlayerEntity* camera, const ItemSet& filter) const {
	// Just ignore too close to let natural highlight take over
	if (Vec3::distance(loot->origin, camera->origin) < 50.0) {
		return;
	}
	// Filter which loot to highlight
	const size_t index = static_cast<uint32_t>(loot->custom_script_int);
	const bool active = index < filter.size() ? filter.test(index) : false;
	const uint32_t style = active ? 0x51408A89 : 0x54208787;
	const float time = ctx.state.curtime + 30.0f;
	if (ctx.entity_check(loot)) {
		// For some reason this is super fucking slow...
		ctx.process.write<bool>(loot->address + 0x3E0, true);
		ctx.process.write<float>(loot->address + 0x33C, config.distance);
		ctx.process.write<float>(loot->address + 0x314, time);
		ctx.process.write<uint32_t>(loot->address + 0x2A8, style);
	}
}

void Highlight::update_filter(const GameState& state, const PlayerEntity* camera, ItemSet& filter) {
	player_items(camera, filter);
	// Update the last seen weapon relevant weapon
	if (auto weapon = state.get_entity<WeaponXEntity>(camera->active_weapon())) {
		// Ignore bare hands as weapon used to filter loot
		if (state.weapon_is_melee(weapon->weapon_index)) {
			// Try last seen weapon instead
			if (const auto last_weapon = state.get_entity<WeaponXEntity>(weapon_handle)) {
				weapon = last_weapon;
			}
		}
		else {
			weapon_handle = weapon->handle;
		}
		// Add relevant loot for this weapon
		weapon_attachments(state.weapon_name(weapon->weapon_index), filter);
	}
	else {
		weapon_handle = EHandle{};
	}
}

inline static void bit_set(ItemSet& filter, ItemID item) {
	filter.set(static_cast<uint32_t>(item));
}
void Highlight::player_items(const PlayerEntity* player, ItemSet& filter) {
	// TODO: Figure out if you already have these
	bit_set(filter, ItemID::KNOCKDOWN_SHIELD_LV3);
	bit_set(filter, ItemID::KNOCKDOWN_SHIELD_LV4);
	bit_set(filter, ItemID::BACKPACK_LV2);
	bit_set(filter, ItemID::BACKPACK_LV3);
	bit_set(filter, ItemID::BACKPACK_LV4);

	// Healing
	if (player->health < player->max_health) {
		bit_set(filter, ItemID::SYRINGE);
	}
	bit_set(filter, ItemID::SHIELD_CELL);
	bit_set(filter, ItemID::SHIELD_BATTERY);
	bit_set(filter, ItemID::MED_KIT);
	bit_set(filter, ItemID::PHOENIX_KIT);

	// Helmets
	bit_set(filter, ItemID::HELMET_LV4);
	if (player->helmet_type < 3) {
		bit_set(filter, ItemID::HELMET_LV3);
		if (player->helmet_type < 2) {
			bit_set(filter, ItemID::HELMET_LV2);
			if (player->helmet_type < 1) {
				bit_set(filter, ItemID::HELMET_LV1);
			}
		}
	}

	// Body armors
	const bool needs_shields = player->shields < player->max_shields;
	if (player->armor_type < 1 || player->armor_type == 1 && needs_shields) {
		bit_set(filter, ItemID::BODY_ARMOR_LV1);
		bit_set(filter, ItemID::EVO_SHIELD_LV1);
	}
	if (player->armor_type < 2 || player->armor_type == 2 && needs_shields) {
		bit_set(filter, ItemID::BODY_ARMOR_LV2);
		bit_set(filter, ItemID::EVO_SHIELD_LV2);
	}
	bit_set(filter, ItemID::BODY_ARMOR_LV3);
	bit_set(filter, ItemID::BODY_ARMOR_LV4);
	bit_set(filter, ItemID::EVO_SHIELD_LV3);
	bit_set(filter, ItemID::EVO_SHIELD_LV4);
}
void Highlight::weapon_attachments(WeaponName weapon_name, ItemSet& filter) {
	filter |= weapon_set(weapon_name);
}
