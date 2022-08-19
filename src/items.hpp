#pragma once

#include "hash.hpp"

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
	SpecialLightAmmo,
};

enum class Rarity: uint8_t {
	Common,
	Rare,
	Epic,
	Legendary,
	Heirloom,
};

enum class WeaponIndex: uint32_t {
	R301 = 49,
	SENTINEL = 52,
	BOCEK = 55,
	R99 = 142,
	P2020,
	HEMLOK = 145,
	LSTAR = 148,
	CHARGE_RIFLE = 183,
	G7_SCOUT = 216,
	EVA8_AUTO = 505,
	ALTERNATOR = 562,
	WINGMAN = 620,
	MASTIFF = 748,
	FLATLINE = 1049,
	HAVOC = 1095,
	REPEATER = 1144,
	RE45 = 1927,
	DEVOTION = 1960,
	LONGBOW = 1976,
	KRABER = 2096,
	MOZAMBIQUE = 2132,
	PROWLER = 2142,
	PEACEKEEPER = 2162,
	SPITFIRE = 2267,
	TRIPLE_TAKE = 2293,
	VOLT = 2342,
};

enum class WeaponName: uint32_t {
	R301 = hash("mp_weapon_rspn101"),
	SENTINEL = hash("mp_weapon_sentinel"),
	BOCEK = hash("mp_weapon_bow"),
	MELEE_SURVIVAL = hash("mp_weapon_melee_survival"),
	ALTERNATOR = hash("mp_weapon_alternator_smg"),
	RE45 = hash("mp_weapon_autopistol"),
	CHARGE_RIFLE = hash("mp_weapon_defender"),
	DEVOTION = hash("mp_weapon_esaw"),
	LONGBOW = hash("mp_weapon_dmr"),
	HAVOC = hash("mp_weapon_energy_ar"),
	EVA8_AUTO = hash("mp_weapon_shotgun"),
	FLATLINE = hash("mp_weapon_vinson"),
	G7_SCOUT = hash("mp_weapon_g2"),
	HEMLOK = hash("mp_weapon_hemlok"),
	KRABER = hash("mp_weapon_sniper"),
	LSTAR = hash("mp_weapon_lstar"),
	MASTIFF = hash("mp_weapon_mastiff"),
	MOZAMBIQUE = hash("mp_weapon_shotgun_pistol"),
	PROWLER = hash("mp_weapon_pdw"),
	PEACEKEEPER = hash("mp_weapon_energy_shotgun"),
	R99 = hash("mp_weapon_r97"),
	P2020 = hash("mp_weapon_semipistol"),
	SPITFIRE = hash("mp_weapon_lmg"),
	TRIPLE_TAKE = hash("mp_weapon_doubletake"),
	WINGMAN = hash("mp_weapon_wingman"),
	VOLT = hash("mp_weapon_volt_smg"),
	REPEATER = hash("mp_weapon_3030"),

	EMPLACED_MINIGUN = hash("mp_weapon_mounted_turret_weapon"),
	CLUSTER_BOMB_LAUNCHER = hash("mp_weapon_cluster_bomb_launcher"),

	CONSUMABLE = hash("mp_ability_consumable"),

	THERMITE_GRENADE = hash("mp_weapon_thermite_grenade"),
	FRAG_GRENADE = hash("mp_weapon_frag_grenade"),
	ARC_STAR = hash("mp_weapon_grenade_emp"),
};

enum class ItemID : uint32_t {
	LIGHT_ROUNDS = 124,
	ENERGY_AMMO,
	SHOTGUN_SHELLS,
	HEAVY_ROUNDS,
	SNIPER_AMMO,
	ARROWS,

	ULTIMATE_ACCELERANT = 164,
	PHOENIX_KIT,
	MED_KIT,
	SYRINGE,
	SHIELD_BATTERY,
	SHIELD_CELL,

	HELMET_LV1 = 170,
	HELMET_LV2,
	HELMET_LV3,
	HELMET_LV4,
	BODY_ARMOR_LV1,
	BODY_ARMOR_LV2,
	BODY_ARMOR_LV3,
	BODY_ARMOR_LV4,
	EVO_SHIELD_LV0,
	EVO_SHIELD_LV1,
	EVO_SHIELD_LV2,
	EVO_SHIELD_LV3,
	EVO_SHIELD_LV4,

	KNOCKDOWN_SHIELD_LV1 = 184,
	KNOCKDOWN_SHIELD_LV2,
	KNOCKDOWN_SHIELD_LV3,
	KNOCKDOWN_SHIELD_LV4,
	BACKPACK_LV1,
	BACKPACK_LV2,
	BACKPACK_LV3,
	BACKPACK_LV4,

	THERMITE_GRENADE = 192,
	FRAG_GRENADE,
	ARC_STAR,

	HCOG_CLASSIC = 195,
	HCOG_BRUISER,
	HOLO,
	VARIABLE_HOLO,
	DIGITAL_THREAT,
	HCOG_RANGER,
	VARIABLE_AOG,
	SNIPER,
	VARIABLE_SNIPER,
	DIGITAL_SNIPER_THREAT,

	BARREL_STABILIZER_LV1 = 205,
	BARREL_STABILIZER_LV2,
	BARREL_STABILIZER_LV3,
	BARREL_STABILIZER_LV4,
	LIGHT_MAGAZINE_LV1,
	LIGHT_MAGAZINE_LV2,
	LIGHT_MAGAZINE_LV3,
	LIGHT_MAGAZINE_LV4,
	HEAVY_MAGAZINE_LV1,
	HEAVY_MAGAZINE_LV2,
	HEAVY_MAGAZINE_LV3,
	HEAVY_MAGAZINE_LV4,
	ENERGY_MAGAZINE_LV1,
	ENERGY_MAGAZINE_LV2,
	ENERGY_MAGAZINE_LV3,
	ENERGY_MAGAZINE_LV4,
	SNIPER_MAGAZINE_LV1,
	SNIPER_MAGAZINE_LV2,
	SNIPER_MAGAZINE_LV3,
	SNIPER_MAGAZINE_LV4,
	SHOTGUN_BOLT_LV1,
	SHOTGUN_BOLT_LV2,
	SHOTGUN_BOLT_LV3,
	STANDARD_STOCK_LV1,
	STANDARD_STOCK_LV2,
	STANDARD_STOCK_LV3,
	SNIPER_STOCK_LV1,
	SNIPER_STOCK_LV2,
	SNIPER_STOCK_LV3,

	TURBOCHARGER = 234,
	SKULLPIERCER_RIFLING = 237,
	HAMMERPOINT_ROUNDS = 238,
	ANVIL_RECEIVER = 239,

	DEADEYE_TEMPO = 245,
	QUICKDRAW_HOLSTER = 246,
	SHATTER_CAPS = 247,
	KINETIC_FEEDER = 248,
	BOOSTED_LOADER = 249,

	// VAULT_KEY = 232,

	// TREASURE_PACK = 234,

	// HEAT_SHIELD = 237,
	// MOBILE_RESPAWN_BEACON = 238,
	// MRVN_ARM = 239,
	// CORRUPTED_KEYCARD = 240,
};

using ItemSet = std::bitset<256>;

// Returns the set of attachment items compatible with this weapon.
ItemSet weapon_set(WeaponName weapon_name);
