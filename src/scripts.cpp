#include "scripts.hpp"
#include "context.hpp"
#include "state.hpp"
#include "entities.hpp"

void Scripts::run(GameContext& ctx) {
	bunnyhop(ctx);
	rapidfire(ctx);
	autoreload(ctx);
	sentinel(ctx);
}

void Scripts::bunnyhop(GameContext& ctx) {
	if (!config.bunnyhop) {
		return;
	}
	ctx.jump.force = true;
	// Very simply bunnyhop:
	// Force the jump button to be released while airborne
	const auto local = ctx.state.local_player();
	if (local && !local->is_onground()) {
		ctx.jump.release = true;
	}
}
void Scripts::rapidfire(GameContext& ctx) {
	if (!config.rapidfire) {
		return;
	}
	ctx.attack.force = true;
	const auto weapon = ctx.state.active_weapon();
	if (!weapon) {
		return;
	}
	bool rapidfire;
	switch (ctx.state.weapon_name(weapon->weapon_index)) {
		case WeaponName::P2020: rapidfire = true; break;
		case WeaponName::G7_SCOUT: rapidfire = true; break;
		// TODO: Single fire weapons Hemlok and R301
		default: rapidfire = false; break;
	}
	if (!rapidfire) {
		return;
	}
	ctx.attack.release = ctx.rapidfire();
}
void Scripts::autoreload(GameContext& ctx) {
	if (!config.autoreload) {
		return;
	}
	ctx.attack.force = true;
	ctx.reload.force = true;
	const auto weapon = ctx.state.active_weapon();
	if (!weapon) {
		return;
	}
	int32_t min_ammo;
	switch (ctx.state.weapon_name(weapon->weapon_index)) {
		case WeaponName::VOLT: min_ammo = 1; break;
		case WeaponName::HAVOC: min_ammo = 0; break;
		case WeaponName::LSTAR: return;
		case WeaponName::DEVOTION: min_ammo = 1; break;

		case WeaponName::KRABER: min_ammo = 0; break;
		case WeaponName::R99: min_ammo = 1; break;
		case WeaponName::PEACEKEEPER: min_ammo = 0; break;

		case WeaponName::SENTINEL: min_ammo = 0; break;
		case WeaponName::CHARGE_RIFLE: min_ammo = 0; break;
		case WeaponName::LONGBOW: min_ammo = 0; break;
		case WeaponName::TRIPLE_TAKE: min_ammo = 0; break;

		case WeaponName::WINGMAN: min_ammo = 0; break;
		case WeaponName::SPITFIRE: min_ammo = 1; break;
		case WeaponName::PROWLER: min_ammo = 1; break;
		case WeaponName::HEMLOK: min_ammo = 0; break;
		case WeaponName::FLATLINE: min_ammo = 1; break;

		case WeaponName::RE45: min_ammo = 1; break;
		case WeaponName::P2020: min_ammo = 0; break;
		case WeaponName::R301: min_ammo = 1; break;
		case WeaponName::ALTERNATOR: min_ammo = 1; break;
		case WeaponName::G7_SCOUT: min_ammo = 0; break;

		case WeaponName::MOZAMBIQUE: min_ammo = 0; break;
		case WeaponName::EVA8_AUTO: min_ammo = 0; break;
		case WeaponName::MASTIFF: min_ammo = 0; break;

		default: return;
	}
	// Autoreload when ammo reaches the gun's min ammo
	// Guns which reload slower from empty start reloading at 1 ammo left
	if (weapon->ammo_in_clip <= min_ammo) {
		ctx.attack.release = true;
		ctx.reload.press = true;
	}
}
void Scripts::sentinel(GameContext& ctx) {
	if (!config.sentinel) {
		return;
	}
	const auto weapon = ctx.state.active_weapon();
	if (!weapon) {
		return;
	}
	if (ctx.state.weapon_name(weapon->weapon_index) != WeaponName::SENTINEL) {
		return;
	}
	ctx.reload.force = true;
	if (ctx.attack.state) {
		ctx.reload.press = ctx.rapidfire();
	}
}
