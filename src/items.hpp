#pragma once

#include <cstdint>
#include <bitset>

enum class AmmoType: uint8_t {
	LightRounds,
	EnergyAmmo,
	ShotgunShells,
	HeavyRounds,
	SpecialHeavyRounds,
	SpecialShotgunShells,
	ExperimentalAmmo,
};

enum class Rarity: uint8_t {
	Common,
	Rare,
	Epic,
	Legendary,
	Special,
};

enum class WeaponID: uint32_t {
	BARE_HANDS = 16,

	TRIPLE_TAKE = 58,
	DEVOTION = 44,
	HAVOC = 60,
	CHARGE_RIFLE = 62,

	WINGMAN = 59,
	SPITFIRE = 57,
	LONGBOW = 45,
	PROWLER = 53,
	HEMLOCK = 49,
	FLATLINE = 47,

	RE45 = 43,
	P2020 = 56,
	R301 = 0,
	R99 = 55,
	ALTERNATOR = 42,
	G7_SCOUT = 48,

	MOZAMBIQUE = 52,
	PEACEKEEPER = 54,
	EVA8_AUTO = 46,

	LSTAR = 61,
	MASTIFF = 51,
	KRABER = 50,
};

enum class ItemID : uint32_t {
	GOLD_KRABER = 1,
	GOLD_MASTIFF = 2,
	GOLD_LSTAR = 3,
	HAVOC = 4,
	GOLD_HAVOC = 5,
	DEVOTION = 6,
	TRIPLE_TAKE = 8,
	GOLD_TRIPLE_TAKE = 9,
	FLATLINE = 10,
	GOLD_FLATLINE = 11,
	HEMLOCK = 12,
	G7_SCOUT = 13,
	GOLD_G7_SCOUT = 14,
	ALTERNATOR = 15,
	GOLD_ALTERNATOR = 16,
	R99 = 17,
	PROWLER = 18,
	GOLD_PROWLER = 19,
	LONGBOW = 20,
	GOLD_LONGBOW = 21,
	CHARGE_RIFLE = 22,
	GOLD_CHARGE_RIFLE = 23,
	SPITFIRE = 24,
	R301 = 25,
	EVA8_AUTO = 26,
	PEACEKEEPER = 29,
	GOLD_PEACEKEEPER = 30,
	MOZAMBIQUE = 31,
	GOLD_MOZAMBIQUE = 32,
	WINGMAN = 33,
	GOLD_WINGMAN = 34,
	P2020 = 35,
	GOLD_P2020 = 36,
	RE45 = 37,
	GOLD_RE45 = 38,

	LIGHT_ROUNDS,
	ENERGY_AMMO,
	SHOTGUN_SHELLS,
	HEAVY_ROUNDS,

	ULTIMATE_ACCELERANT,
	PHOENIX_KIT,
	MED_KIT,
	SYRINGE,
	SHIELD_BATTERY,
	SHIELD_CELL,

	HELMET_LV1,
	HELMET_LV2,
	HELMET_LV3,
	HELMET_LV4,
	BODY_ARMOR_LV1,
	BODY_ARMOR_LV2,
	BODY_ARMOR_LV3,
	BODY_ARMOR_LV4,
	KNOCKDOWN_SHIELD_LV1,
	KNOCKDOWN_SHIELD_LV2,
	KNOCKDOWN_SHIELD_LV3,
	KNOCKDOWN_SHIELD_LV4,
	BACKPACK_LV1,
	BACKPACK_LV2,
	BACKPACK_LV3,
	BACKPACK_LV4,

	THERMITE_GRENADE,
	FRAG_GRENADE,
	ARC_STAR,

	HCOG_CLASSIC,
	HCOG_BRUISER,
	HOLO,
	VARIABLE_HOLO,
	DIGITAL_THREAT,
	HCOG_RANGER,
	VARIABLE_AOG,
	SNIPER,
	VARIABLE_SNIPER,
	DIGITAL_SNIPER_THREAT,

	BARREL_STABILIZER_LV1,
	BARREL_STABILIZER_LV2,
	BARREL_STABILIZER_LV3,
	BARREL_STABILIZER_LV4,
	LIGHT_MAGAZINE_LV1,
	LIGHT_MAGAZINE_LV2,
	LIGHT_MAGAZINE_LV3,
	HEAVY_MAGAZINE_LV1,
	HEAVY_MAGAZINE_LV2,
	HEAVY_MAGAZINE_LV3,
	// ENERGY_MAGAZINE_LV1,
	// ENERGY_MAGAZINE_LV2,
	// ENERGY_MAGAZINE_LV3,
	SHOTGUN_BOLT_LV1,
	SHOTGUN_BOLT_LV2,
	SHOTGUN_BOLT_LV3,
	STANDARD_STOCK_LV1,
	STANDARD_STOCK_LV2,
	STANDARD_STOCK_LV3,
	SNIPER_STOCK_LV1,
	SNIPER_STOCK_LV2,
	SNIPER_STOCK_LV3,

	TURBOCHARGER,
	SELECTFIRE_RECEIVER,
	PRECISION_CHOKE,
	// SKULLPIERCER_RIFLING,
	HAMMERPOINT_ROUNDS,
	ANVIL_RECEIVER,
	DOUBLE_TAP_TRIGGER,
	VAULT_KEY,
};

using ItemSet = std::bitset<128>;

// Returns the ItemID for the AmmoType.
ItemID ammo_item(AmmoType ammo);
// Stringifies the AmmoType type.
const char* ammo_str(AmmoType ammo);
// Stringifies the Rarity level.
const char* rarity_str(Rarity rarity);
// Stringifies the ItemID.
const char* item_str(ItemID item);
// Returns the Rarity for the ItemID.
Rarity item_rarity(ItemID item);
// Converts the ItemID to a flag.
ItemSet item_flag(ItemID item);
// Stringifies the WeaponID.
const char* weapon_str(WeaponID weapon);
// Returns the set of ItemID attachments compatible with this weapon.
ItemSet weapon_set(WeaponID weapon);
// Returns the AmmoType for the WeaponID.
AmmoType weapon_ammo(WeaponID weapon);
