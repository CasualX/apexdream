#pragma once

#include <cstdint>
#include <bitset>

enum class AmmoType: uint8_t {
	LightRounds,
	EnergyAmmo,
	ShotgunShells,
	HeavyRounds,
	SniperAmmo,
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
	BARE_HANDS = 15,

	HAVOC = 63,
	LSTAR = 64,

	KRABER = 53,
	MASTIFF = 54,
	DEVOTION = 47,

	SENTINEL = 1,
	CHARGE_RIFLE = 65,
	LONGBOW = 48,
	TRIPLE_TAKE = 61,

	WINGMAN = 62,
	SPITFIRE = 60,
	PROWLER = 56,
	HEMLOCK = 52,
	FLATLINE = 50,

	RE45 = 46,
	P2020 = 59,
	R301 = 0,
	R99 = 58,
	ALTERNATOR = 45,
	G7_SCOUT = 51,

	MOZAMBIQUE = 55,
	PEACEKEEPER = 57,
	EVA8_AUTO = 49,
};

enum class ItemID : uint32_t {
	GOLD_KRABER = 1,
	GOLD_MASTIFF = 2,
	LSTAR = 3,
	HAVOC = 4,
	GOLD_HAVOC = 5,
	DEVOTION = 6,
	TRIPLE_TAKE = 7,
	GOLD_TRIPLE_TAKE = 8,
	FLATLINE = 9,
	GOLD_FLATLINE = 10,
	HEMLOCK = 11,
	G7_SCOUT = 12,
	GOLD_G7_SCOUT = 13,
	ALTERNATOR = 14,
	GOLD_ALTERNATOR = 15,
	R99 = 16,
	PROWLER = 17,
	GOLD_PROWLER = 18,
	LONGBOW = 19,
	GOLD_LONGBOW = 20,
	CHARGE_RIFLE = 21,
	GOLD_CHARGE_RIFLE = 22,
	SPITFIRE = 23,
	R301 = 24,
	GOLD_R301 = 25,
	EVA8_AUTO = 26,
	GOLD_EVA8_AUTO = 27,
	PEACEKEEPER = 28,
	GOLD_PEACEKEEPER = 29,
	MOZAMBIQUE = 30,
	GOLD_MOZAMBIQUE = 31,
	WINGMAN = 32,
	GOLD_WINGMAN = 33,
	P2020 = 34,
	GOLD_P2020 = 35,
	RE45 = 36,
	GOLD_RE45 = 37,

	LIGHT_ROUNDS = 40,
	ENERGY_AMMO,
	SHOTGUN_SHELLS,
	HEAVY_ROUNDS,
	SNIPER_AMMO,

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
	SNIPER_MAGAZINE_LV1,
	SNIPER_MAGAZINE_LV2,
	SNIPER_MAGAZINE_LV3,
	SHOTGUN_BOLT_LV1,
	SHOTGUN_BOLT_LV2,
	SHOTGUN_BOLT_LV3,
	STANDARD_STOCK_LV1,
	STANDARD_STOCK_LV2,
	STANDARD_STOCK_LV3,
	SNIPER_STOCK_LV1,
	SNIPER_STOCK_LV2,
	SNIPER_STOCK_LV3,

	SELECTFIRE_RECEIVER,
	PRECISION_CHOKE,
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
