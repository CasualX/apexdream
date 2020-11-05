#include "config.hpp"
#include "entities.hpp"
#include "state.hpp"
#include "cheats.hpp"

void Config::run(const GameState& state, CheatManager& cheats) {
	// Get the player's active weapon id
	WeaponIndex weapon_index = static_cast<WeaponIndex>(~0u);
	if (const auto local = state.local_player()) {
		if (const auto weapon = state.get_entity<WeaponXEntity>(local->active_weapon())) {
			weapon_index = weapon->weapon_index;
		}
	}
	// If the active weapon id has changed, reload the config
	if (active_id != weapon_index) {
		active_id = weapon_index;
		reload(state.weapon_name(weapon_index), cheats);
	}
}
void Config::reload(WeaponName weapon_name, CheatManager& cheats) const {
	auto& aimassist = cheats.aimassist.config;
	auto& highlight = cheats.highlight.config;

	// Before changing any settings ensure an appropriate default is set
	// Without setting a default a setting will keep its value based on the last active weapon
	// Which is probably not what is intended.

	// Once an appropriate default is selected modify any settings on a per weapon basis:
	switch (weapon_name) {
	case WeaponName::MELEE_SURVIVAL:
		break;

	case WeaponName::VOLT:
		break;
	case WeaponName::DEVOTION:
		break;
	case WeaponName::HAVOC:
		break;
	case WeaponName::LSTAR:
		break;

	case WeaponName::KRABER:
		break;
	case WeaponName::R99:
		break;
	case WeaponName::PEACEKEEPER:
		break;

	case WeaponName::SENTINEL:
		break;
	case WeaponName::CHARGE_RIFLE:
		break;
	case WeaponName::LONGBOW:
		break;
	case WeaponName::TRIPLE_TAKE:
		break;

	case WeaponName::WINGMAN:
		break;
	case WeaponName::SPITFIRE:
		break;
	case WeaponName::PROWLER:
		break;
	case WeaponName::HEMLOK:
		break;
	case WeaponName::FLATLINE:
		break;

	case WeaponName::RE45:
		break;
	case WeaponName::P2020:
		break;
	case WeaponName::R301:
		break;
	case WeaponName::ALTERNATOR:
		break;
	case WeaponName::G7_SCOUT:
		break;

	case WeaponName::MOZAMBIQUE:
		break;
	case WeaponName::EVA8_AUTO:
		break;
	case WeaponName::MASTIFF:
		break;
	}
}
