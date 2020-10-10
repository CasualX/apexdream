#include "items.hpp"

inline static ItemSet item_flag(ItemID item) {
	return ItemSet(1) << static_cast<size_t>(item);
}

ItemSet weapon_set(WeaponName weapon_name) {
	// Compose some helper item sets

	const auto BARREL_STABILIZER =
		item_flag(ItemID::BARREL_STABILIZER_LV1) |
		item_flag(ItemID::BARREL_STABILIZER_LV2) |
		item_flag(ItemID::BARREL_STABILIZER_LV3) |
		item_flag(ItemID::BARREL_STABILIZER_LV4);

	const auto LIGHT_MAGAZINE =
		item_flag(ItemID::LIGHT_MAGAZINE_LV1) |
		item_flag(ItemID::LIGHT_MAGAZINE_LV2) |
		item_flag(ItemID::LIGHT_MAGAZINE_LV3);

	const auto HEAVY_MAGAZINE =
		item_flag(ItemID::HEAVY_MAGAZINE_LV1) |
		item_flag(ItemID::HEAVY_MAGAZINE_LV2) |
		item_flag(ItemID::HEAVY_MAGAZINE_LV3);

	const auto ENERGY_MAGAZINE =
		item_flag(ItemID::ENERGY_MAGAZINE_LV1) |
		item_flag(ItemID::ENERGY_MAGAZINE_LV2) |
		item_flag(ItemID::ENERGY_MAGAZINE_LV3);

	const auto SNIPER_MAGAZINE =
		item_flag(ItemID::SNIPER_MAGAZINE_LV1) |
		item_flag(ItemID::SNIPER_MAGAZINE_LV2) |
		item_flag(ItemID::SNIPER_MAGAZINE_LV3);

	const auto SHOTGUN_BOLT =
		item_flag(ItemID::SHOTGUN_BOLT_LV1) |
		item_flag(ItemID::SHOTGUN_BOLT_LV2) |
		item_flag(ItemID::SHOTGUN_BOLT_LV3);

	const auto STANDARD_STOCK =
		item_flag(ItemID::STANDARD_STOCK_LV1) |
		item_flag(ItemID::STANDARD_STOCK_LV2) |
		item_flag(ItemID::STANDARD_STOCK_LV3);

	const auto SNIPER_STOCK =
		item_flag(ItemID::SNIPER_STOCK_LV1) |
		item_flag(ItemID::SNIPER_STOCK_LV2) |
		item_flag(ItemID::SNIPER_STOCK_LV3);

	const auto SNIPER_OPTICS =
		item_flag(ItemID::HOLO) |
		item_flag(ItemID::HCOG_CLASSIC) |
		item_flag(ItemID::HCOG_BRUISER) |
		item_flag(ItemID::VARIABLE_HOLO) |
		item_flag(ItemID::HCOG_RANGER) |
		item_flag(ItemID::VARIABLE_AOG) |
		item_flag(ItemID::SNIPER) |
		item_flag(ItemID::VARIABLE_SNIPER) |
		item_flag(ItemID::DIGITAL_SNIPER_THREAT);

	const auto AR_OPTICS =
		item_flag(ItemID::HOLO) |
		item_flag(ItemID::HCOG_CLASSIC) |
		item_flag(ItemID::HCOG_BRUISER) |
		item_flag(ItemID::VARIABLE_HOLO) |
		item_flag(ItemID::HCOG_RANGER) |
		item_flag(ItemID::VARIABLE_AOG);

	const auto LMG_OPTICS =
		item_flag(ItemID::HOLO) |
		item_flag(ItemID::HCOG_CLASSIC) |
		item_flag(ItemID::HCOG_BRUISER) |
		item_flag(ItemID::VARIABLE_HOLO) |
		item_flag(ItemID::HCOG_RANGER) |
		item_flag(ItemID::VARIABLE_AOG);

	const auto SMG_OPTICS =
		item_flag(ItemID::HOLO) |
		item_flag(ItemID::HCOG_CLASSIC) |
		item_flag(ItemID::HCOG_BRUISER) |
		item_flag(ItemID::VARIABLE_HOLO) |
		item_flag(ItemID::DIGITAL_THREAT);

	const auto SHOTGUN_OPTICS =
		item_flag(ItemID::HOLO) |
		item_flag(ItemID::HCOG_CLASSIC) |
		item_flag(ItemID::HCOG_BRUISER) |
		item_flag(ItemID::VARIABLE_HOLO) |
		item_flag(ItemID::DIGITAL_THREAT);

	const auto PISTOL_OPTICS =
		item_flag(ItemID::HOLO) |
		item_flag(ItemID::HCOG_CLASSIC) |
		item_flag(ItemID::HCOG_BRUISER) |
		item_flag(ItemID::VARIABLE_HOLO) |
		item_flag(ItemID::DIGITAL_THREAT);

	switch (weapon_name) {
		// Energy weapons

		case WeaponName::VOLT: return
			item_flag(ItemID::ENERGY_AMMO) |
			BARREL_STABILIZER |
			ENERGY_MAGAZINE |
			SMG_OPTICS |
			STANDARD_STOCK;

		case WeaponName::DEVOTION: return
			item_flag(ItemID::ENERGY_AMMO) |
			BARREL_STABILIZER |
			ENERGY_MAGAZINE |
			LMG_OPTICS |
			STANDARD_STOCK |
			item_flag(ItemID::TURBOCHARGER);

		case WeaponName::HAVOC: return
			item_flag(ItemID::ENERGY_AMMO) |
			ENERGY_MAGAZINE |
			AR_OPTICS |
			STANDARD_STOCK |
			item_flag(ItemID::SELECTFIRE_RECEIVER) |
			item_flag(ItemID::TURBOCHARGER);

		case WeaponName::LSTAR: return
			item_flag(ItemID::ENERGY_AMMO) |
			LMG_OPTICS |
			STANDARD_STOCK;

		// Heirloom weapons

		case WeaponName::KRABER: return ItemSet();
		case WeaponName::R99: return ItemSet();
		case WeaponName::PEACEKEEPER: return ItemSet();

		// Sniper weapons

		case WeaponName::SENTINEL: return
			item_flag(ItemID::SNIPER_AMMO) |
			SNIPER_MAGAZINE |
			SNIPER_OPTICS |
			SNIPER_STOCK;

		case WeaponName::CHARGE_RIFLE: return
			item_flag(ItemID::SNIPER_AMMO) |
			SNIPER_OPTICS |
			SNIPER_STOCK;

		case WeaponName::LONGBOW: return
			item_flag(ItemID::SNIPER_AMMO) |
			BARREL_STABILIZER |
			SNIPER_MAGAZINE |
			SNIPER_OPTICS |
			SNIPER_STOCK |
			item_flag(ItemID::SKULLPIERCER_RIFLING);

		case WeaponName::TRIPLE_TAKE: return
			item_flag(ItemID::SNIPER_AMMO) |
			SNIPER_MAGAZINE |
			SNIPER_OPTICS |
			SNIPER_STOCK;

		// Heavy weapons

		case WeaponName::WINGMAN: return
			item_flag(ItemID::HEAVY_ROUNDS) |
			HEAVY_MAGAZINE |
			PISTOL_OPTICS |
			item_flag(ItemID::SKULLPIERCER_RIFLING);

		case WeaponName::SPITFIRE: return
			item_flag(ItemID::HEAVY_ROUNDS) |
			BARREL_STABILIZER |
			HEAVY_MAGAZINE |
			LMG_OPTICS |
			STANDARD_STOCK;

		case WeaponName::PROWLER: return
			item_flag(ItemID::HEAVY_ROUNDS) |
			HEAVY_MAGAZINE |
			SMG_OPTICS |
			STANDARD_STOCK |
			item_flag(ItemID::SELECTFIRE_RECEIVER);

		case WeaponName::HEMLOK: return
			item_flag(ItemID::HEAVY_ROUNDS) |
			BARREL_STABILIZER |
			HEAVY_MAGAZINE |
			AR_OPTICS |
			STANDARD_STOCK;

		case WeaponName::FLATLINE: return
			item_flag(ItemID::HEAVY_ROUNDS) |
			HEAVY_MAGAZINE |
			AR_OPTICS |
			STANDARD_STOCK;

		// Light weapons

		case WeaponName::RE45: return
			item_flag(ItemID::LIGHT_ROUNDS) |
			BARREL_STABILIZER |
			LIGHT_MAGAZINE |
			PISTOL_OPTICS;

		case WeaponName::P2020: return
			item_flag(ItemID::LIGHT_ROUNDS) |
			LIGHT_MAGAZINE |
			PISTOL_OPTICS |
			item_flag(ItemID::HAMMERPOINT_ROUNDS);

		case WeaponName::R301: return
			item_flag(ItemID::LIGHT_ROUNDS) |
			BARREL_STABILIZER |
			LIGHT_MAGAZINE |
			AR_OPTICS |
			STANDARD_STOCK;

		case WeaponName::ALTERNATOR: return
			item_flag(ItemID::LIGHT_ROUNDS) |
			BARREL_STABILIZER |
			LIGHT_MAGAZINE |
			SMG_OPTICS |
			STANDARD_STOCK;

		case WeaponName::G7_SCOUT: return
			item_flag(ItemID::LIGHT_ROUNDS) |
			BARREL_STABILIZER |
			LIGHT_MAGAZINE |
			AR_OPTICS |
			STANDARD_STOCK |
			item_flag(ItemID::DOUBLE_TAP_TRIGGER);

		// Shotgun weapons

		case WeaponName::MOZAMBIQUE: return
			item_flag(ItemID::SHOTGUN_SHELLS) |
			SHOTGUN_BOLT |
			SHOTGUN_OPTICS |
			item_flag(ItemID::HAMMERPOINT_ROUNDS);

		case WeaponName::EVA8_AUTO: return
			item_flag(ItemID::SHOTGUN_SHELLS) |
			SHOTGUN_BOLT |
			SHOTGUN_OPTICS |
			item_flag(ItemID::DOUBLE_TAP_TRIGGER);

		case WeaponName::MASTIFF: return
			item_flag(ItemID::SHOTGUN_SHELLS) |
			SHOTGUN_BOLT |
			SHOTGUN_OPTICS;

		default: return ItemSet();
	}
}
