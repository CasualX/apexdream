#include "config.hpp"
#include "entities.hpp"
#include "state.hpp"
#include "cheats.hpp"

void Config::run(const GameState& state, CheatManager& cheats) {
	// Get the player's active weapon id
	WeaponID weapon_id = static_cast<WeaponID>(~0u);
	if (const auto local = state.local_player()) {
		if (const auto weapon = state.get_entity<WeaponXEntity>(local->active_weapon())) {
			weapon_id = weapon->weapon_name_index;
		}
	}
	// If the active weapon id has changed, reload the config
	if (active_id != weapon_id) {
		active_id = weapon_id;
		reload(weapon_id, cheats);
	}
}
void Config::reload(WeaponID weapon, CheatManager& cheats) const {
	auto& aimassist = cheats.aimassist.config;
	auto& highlight = cheats.highlight.config;

	// Before changing any settings ensure an appropriate default is set
	// Without setting a default a setting will keep its value based on the last active weapon
	// Which is probably not what is intended.

	// Once an appropriate default is selected modify any settings on a per weapon basis:
	switch (weapon) {
	case WeaponID::BARE_HANDS:
		break;

	case WeaponID::TRIPLE_TAKE:
		break;
	case WeaponID::DEVOTION:
		break;
	case WeaponID::HAVOC:
		break;
	case WeaponID::CHARGE_RIFLE:
		break;

	case WeaponID::WINGMAN:
		break;
	case WeaponID::SPITFIRE:
		break;
	case WeaponID::LONGBOW:
		break;
	case WeaponID::PROWLER:
		break;
	case WeaponID::HEMLOCK:
		break;
	case WeaponID::FLATLINE:
		break;

	case WeaponID::RE45:
		break;
	case WeaponID::P2020:
		break;
	case WeaponID::R301:
		break;
	case WeaponID::R99:
		break;
	case WeaponID::ALTERNATOR:
		break;
	case WeaponID::G7_SCOUT:
		break;

	case WeaponID::MOZAMBIQUE:
		break;
	case WeaponID::PEACEKEEPER:
		break;
	case WeaponID::EVA8_AUTO:
		break;

	case WeaponID::LSTAR:
		break;
	case WeaponID::MASTIFF:
		break;
	case WeaponID::KRABER:
		break;
	}
}
