#include "items.hpp"

ItemID ammo_item(AmmoType ammo) {
	switch (ammo) {
		case AmmoType::LightRounds: return ItemID::LIGHT_ROUNDS;
		case AmmoType::EnergyAmmo: return ItemID::ENERGY_AMMO;
		case AmmoType::ShotgunShells: return ItemID::SHOTGUN_SHELLS;
		case AmmoType::HeavyRounds: return ItemID::HEAVY_ROUNDS;
		case AmmoType::SniperAmmo: return ItemID::SNIPER_AMMO;
		default: return static_cast<ItemID>(0);
	}
}

const char* ammo_str(AmmoType ammo) {
	switch (ammo) {
		case AmmoType::LightRounds: return "LightRounds";
		case AmmoType::EnergyAmmo: return "EnergyAmmo";
		case AmmoType::ShotgunShells: return "ShotgunShells";
		case AmmoType::HeavyRounds: return "HeavyRounds";
		case AmmoType::SniperAmmo: return "SniperAmmo";
		case AmmoType::SpecialHeavyRounds: return "SpecialHeavyRounds";
		case AmmoType::SpecialShotgunShells: return "SpecialShotgunShells";
		case AmmoType::ExperimentalAmmo: return "ExperimentalAmmo";
		default: return nullptr;
	}
}

const char* rarity_str(Rarity rarity) {
	switch (rarity) {
		case Rarity::Common: return "Common";
		case Rarity::Rare: return "Rare";
		case Rarity::Epic: return "Epic";
		case Rarity::Legendary: return "Legendary";
		case Rarity::Special: return "Special";
		default: return nullptr;
	}
}

const char* item_str(ItemID item) {
	switch (item) {
		case ItemID::GOLD_KRABER: return "GOLD_KRABER";
		case ItemID::GOLD_MASTIFF: return "GOLD_MASTIFF";
		case ItemID::LSTAR: return "LSTAR";
		case ItemID::HAVOC: return "HAVOC";
		case ItemID::GOLD_HAVOC: return "GOLD_HAVOC";
		case ItemID::DEVOTION: return "DEVOTION";
		case ItemID::TRIPLE_TAKE: return "TRIPLE_TAKE";
		case ItemID::GOLD_TRIPLE_TAKE: return "GOLD_TRIPLE_TAKE";
		case ItemID::FLATLINE: return "FLATLINE";
		case ItemID::GOLD_FLATLINE: return "GOLD_FLATLINE";
		case ItemID::HEMLOCK: return "HEMLOCK";
		case ItemID::G7_SCOUT: return "G7_SCOUT";
		case ItemID::GOLD_G7_SCOUT: return "GOLD_G7_SCOUT";
		case ItemID::ALTERNATOR: return "ALTERNATOR";
		case ItemID::GOLD_ALTERNATOR: return "GOLD_ALTERNATOR";
		case ItemID::R99: return "R99";
		case ItemID::PROWLER: return "PROWLER";
		case ItemID::GOLD_PROWLER: return "GOLD_PROWLER";
		case ItemID::LONGBOW: return "LONGBOW";
		case ItemID::GOLD_LONGBOW: return "GOLD_LONGBOW";
		case ItemID::CHARGE_RIFLE: return "CHARGE_RIFLE";
		case ItemID::GOLD_CHARGE_RIFLE: return "GOLD_CHARGE_RIFLE";
		case ItemID::SPITFIRE: return "SPITFIRE";
		case ItemID::R301: return "R301";
		case ItemID::GOLD_R301: return "GOLD_R301";
		case ItemID::EVA8_AUTO: return "EVA8_AUTO";
		case ItemID::PEACEKEEPER: return "PEACEKEEPER";
		case ItemID::GOLD_PEACEKEEPER: return "GOLD_PEACEKEEPER";
		case ItemID::MOZAMBIQUE: return "MOZAMBIQUE";
		case ItemID::GOLD_MOZAMBIQUE: return "GOLD_MOZAMBIQUE";
		case ItemID::WINGMAN: return "WINGMAN";
		case ItemID::GOLD_WINGMAN: return "GOLD_WINGMAN";
		case ItemID::P2020: return "P2020";
		case ItemID::GOLD_P2020: return "GOLD_P2020";
		case ItemID::RE45: return "RE45";
		case ItemID::GOLD_RE45: return "GOLD_RE45";
		case ItemID::SENTINEL: return "SENTINEL";
		case ItemID::GOLD_SENTINEL: return "GOLD_SENTINEL";

		case ItemID::LIGHT_ROUNDS: return "LIGHT_ROUNDS";
		case ItemID::ENERGY_AMMO: return "ENERGY_AMMO";
		case ItemID::SHOTGUN_SHELLS: return "SHOTGUN_SHELLS";
		case ItemID::HEAVY_ROUNDS: return "HEAVY_ROUNDS";
		case ItemID::SNIPER_AMMO: return "SNIPER_AMMO";

		case ItemID::ULTIMATE_ACCELERANT: return "ULTIMATE_ACCELERANT";
		case ItemID::PHOENIX_KIT: return "PHOENIX_KIT";
		case ItemID::MED_KIT: return "MED_KIT";
		case ItemID::SYRINGE: return "SYRINGE";
		case ItemID::SHIELD_BATTERY: return "SHIELD_BATTERY";
		case ItemID::SHIELD_CELL: return "SHIELD_CELL";

		case ItemID::HELMET_LV1: return "HELMET_LV1";
		case ItemID::HELMET_LV2: return "HELMET_LV2";
		case ItemID::HELMET_LV3: return "HELMET_LV3";
		case ItemID::HELMET_LV4: return "HELMET_LV4";
		case ItemID::BODY_ARMOR_LV1: return "BODY_ARMOR_LV1";
		case ItemID::BODY_ARMOR_LV2: return "BODY_ARMOR_LV2";
		case ItemID::BODY_ARMOR_LV3: return "BODY_ARMOR_LV3";
		case ItemID::BODY_ARMOR_LV4: return "BODY_ARMOR_LV4";
		case ItemID::EVO_SHIELD_LV1: return "EVO_SHIELD_LV1";
		case ItemID::EVO_SHIELD_LV2: return "EVO_SHIELD_LV2";
		case ItemID::EVO_SHIELD_LV3: return "EVO_SHIELD_LV3";
		case ItemID::EVO_SHIELD_LV4: return "EVO_SHIELD_LV4";
		case ItemID::KNOCKDOWN_SHIELD_LV1: return "KNOCKDOWN_SHIELD_LV1";
		case ItemID::KNOCKDOWN_SHIELD_LV2: return "KNOCKDOWN_SHIELD_LV2";
		case ItemID::KNOCKDOWN_SHIELD_LV3: return "KNOCKDOWN_SHIELD_LV3";
		case ItemID::KNOCKDOWN_SHIELD_LV4: return "KNOCKDOWN_SHIELD_LV4";
		case ItemID::BACKPACK_LV1: return "BACKPACK_LV1";
		case ItemID::BACKPACK_LV2: return "BACKPACK_LV2";
		case ItemID::BACKPACK_LV3: return "BACKPACK_LV3";
		case ItemID::BACKPACK_LV4: return "BACKPACK_LV4";

		case ItemID::THERMITE_GRENADE: return "THERMITE_GRENADE";
		case ItemID::FRAG_GRENADE: return "FRAG_GRENADE";
		case ItemID::ARC_STAR: return "ARC_STAR";

		case ItemID::HCOG_CLASSIC: return "HCOG_CLASSIC";
		case ItemID::HCOG_BRUISER: return "HCOG_BRUISER";
		case ItemID::HOLO: return "HOLO";
		case ItemID::VARIABLE_HOLO: return "VARIABLE_HOLO";
		case ItemID::DIGITAL_THREAT: return "DIGITAL_THREAT";
		case ItemID::HCOG_RANGER: return "HCOG_RANGER";
		case ItemID::VARIABLE_AOG: return "VARIABLE_AOG";
		case ItemID::SNIPER: return "SNIPER";
		case ItemID::VARIABLE_SNIPER: return "VARIABLE_SNIPER";
		case ItemID::DIGITAL_SNIPER_THREAT: return "DIGITAL_SNIPER_THREAT";

		case ItemID::BARREL_STABILIZER_LV1: return "BARREL_STABILIZER_LV1";
		case ItemID::BARREL_STABILIZER_LV2: return "BARREL_STABILIZER_LV2";
		case ItemID::BARREL_STABILIZER_LV3: return "BARREL_STABILIZER_LV3";
		case ItemID::BARREL_STABILIZER_LV4: return "BARREL_STABILIZER_LV4";
		case ItemID::LIGHT_MAGAZINE_LV1: return "LIGHT_MAGAZINE_LV1";
		case ItemID::LIGHT_MAGAZINE_LV2: return "LIGHT_MAGAZINE_LV2";
		case ItemID::LIGHT_MAGAZINE_LV3: return "LIGHT_MAGAZINE_LV3";
		case ItemID::HEAVY_MAGAZINE_LV1: return "HEAVY_MAGAZINE_LV1";
		case ItemID::HEAVY_MAGAZINE_LV2: return "HEAVY_MAGAZINE_LV2";
		case ItemID::HEAVY_MAGAZINE_LV3: return "HEAVY_MAGAZINE_LV3";
		case ItemID::SNIPER_MAGAZINE_LV1: return "SNIPER_MAGAZINE_LV1";
		case ItemID::SNIPER_MAGAZINE_LV2: return "SNIPER_MAGAZINE_LV2";
		case ItemID::SNIPER_MAGAZINE_LV3: return "SNIPER_MAGAZINE_LV3";
		case ItemID::SHOTGUN_BOLT_LV1: return "SHOTGUN_BOLT_LV1";
		case ItemID::SHOTGUN_BOLT_LV2: return "SHOTGUN_BOLT_LV2";
		case ItemID::SHOTGUN_BOLT_LV3: return "SHOTGUN_BOLT_LV3";
		case ItemID::STANDARD_STOCK_LV1: return "STANDARD_STOCK_LV1";
		case ItemID::STANDARD_STOCK_LV2: return "STANDARD_STOCK_LV2";
		case ItemID::STANDARD_STOCK_LV3: return "STANDARD_STOCK_LV3";
		case ItemID::SNIPER_STOCK_LV1: return "SNIPER_STOCK_LV1";
		case ItemID::SNIPER_STOCK_LV2: return "SNIPER_STOCK_LV2";
		case ItemID::SNIPER_STOCK_LV3: return "SNIPER_STOCK_LV3";

		case ItemID::SELECTFIRE_RECEIVER: return "SELECTFIRE_RECEIVER";
		case ItemID::PRECISION_CHOKE: return "PRECISION_CHOKE";
		case ItemID::HAMMERPOINT_ROUNDS: return "HAMMERPOINT_ROUNDS";
		case ItemID::ANVIL_RECEIVER: return "ANVIL_RECEIVER";
		case ItemID::DOUBLE_TAP_TRIGGER: return "DOUBLE_TAP_TRIGGER";
		case ItemID::VAULT_KEY: return "VAULT_KEY";

		default: return nullptr;
	}
}

Rarity item_rarity(ItemID item) {
	switch (item) {
	case ItemID::GOLD_KRABER: return Rarity::Legendary;
	case ItemID::GOLD_MASTIFF: return Rarity::Legendary;
	case ItemID::LSTAR: return Rarity::Common;
	case ItemID::HAVOC: return Rarity::Common;
	case ItemID::GOLD_HAVOC: return Rarity::Legendary;
	case ItemID::DEVOTION: return Rarity::Common;
	case ItemID::TRIPLE_TAKE: return Rarity::Common;
	case ItemID::GOLD_TRIPLE_TAKE: return Rarity::Legendary;
	case ItemID::FLATLINE: return Rarity::Common;
	case ItemID::GOLD_FLATLINE: return Rarity::Legendary;
	case ItemID::HEMLOCK: return Rarity::Common;
	case ItemID::G7_SCOUT: return Rarity::Common;
	case ItemID::GOLD_G7_SCOUT: return Rarity::Legendary;
	case ItemID::ALTERNATOR: return Rarity::Common;
	case ItemID::GOLD_ALTERNATOR: return Rarity::Legendary;
	case ItemID::R99: return Rarity::Common;
	case ItemID::PROWLER: return Rarity::Common;
	case ItemID::GOLD_PROWLER: return Rarity::Legendary;
	case ItemID::LONGBOW: return Rarity::Common;
	case ItemID::GOLD_LONGBOW: return Rarity::Legendary;
	case ItemID::CHARGE_RIFLE: return Rarity::Common;
	case ItemID::GOLD_CHARGE_RIFLE: return Rarity::Legendary;
	case ItemID::SPITFIRE: return Rarity::Common;
	case ItemID::R301: return Rarity::Common;
	case ItemID::GOLD_R301: return Rarity::Legendary;
	case ItemID::EVA8_AUTO: return Rarity::Common;
	case ItemID::PEACEKEEPER: return Rarity::Common;
	case ItemID::GOLD_PEACEKEEPER: return Rarity::Legendary;
	case ItemID::MOZAMBIQUE: return Rarity::Common;
	case ItemID::GOLD_MOZAMBIQUE: return Rarity::Legendary;
	case ItemID::WINGMAN: return Rarity::Common;
	case ItemID::GOLD_WINGMAN: return Rarity::Legendary;
	case ItemID::P2020: return Rarity::Common;
	case ItemID::GOLD_P2020: return Rarity::Legendary;
	case ItemID::RE45: return Rarity::Common;
	case ItemID::GOLD_RE45: return Rarity::Legendary;
	case ItemID::SENTINEL: return Rarity::Common;
	case ItemID::GOLD_SENTINEL: return Rarity::Legendary;

	case ItemID::LIGHT_ROUNDS: return Rarity::Rare;
	case ItemID::ENERGY_AMMO: return Rarity::Rare;
	case ItemID::SHOTGUN_SHELLS: return Rarity::Rare;
	case ItemID::HEAVY_ROUNDS: return Rarity::Rare;
	case ItemID::SNIPER_AMMO: return Rarity::Rare;

	case ItemID::ULTIMATE_ACCELERANT: return Rarity::Common;
	case ItemID::PHOENIX_KIT: return Rarity::Epic;
	case ItemID::MED_KIT: return Rarity::Rare;
	case ItemID::SYRINGE: return Rarity::Common;
	case ItemID::SHIELD_BATTERY: return Rarity::Rare;
	case ItemID::SHIELD_CELL: return Rarity::Common;

	case ItemID::HELMET_LV1: return Rarity::Common;
	case ItemID::HELMET_LV2: return Rarity::Rare;
	case ItemID::HELMET_LV3: return Rarity::Epic;
	case ItemID::HELMET_LV4: return Rarity::Legendary;
	case ItemID::BODY_ARMOR_LV1: return Rarity::Common;
	case ItemID::BODY_ARMOR_LV2: return Rarity::Rare;
	case ItemID::BODY_ARMOR_LV3: return Rarity::Epic;
	case ItemID::BODY_ARMOR_LV4: return Rarity::Legendary;
	case ItemID::EVO_SHIELD_LV1: return Rarity::Common;
	case ItemID::EVO_SHIELD_LV2: return Rarity::Rare;
	case ItemID::EVO_SHIELD_LV3: return Rarity::Epic;
	case ItemID::EVO_SHIELD_LV4: return Rarity::Legendary;
	case ItemID::KNOCKDOWN_SHIELD_LV1: return Rarity::Common;
	case ItemID::KNOCKDOWN_SHIELD_LV2: return Rarity::Rare;
	case ItemID::KNOCKDOWN_SHIELD_LV3: return Rarity::Epic;
	case ItemID::KNOCKDOWN_SHIELD_LV4: return Rarity::Legendary;
	case ItemID::BACKPACK_LV1: return Rarity::Common;
	case ItemID::BACKPACK_LV2: return Rarity::Rare;
	case ItemID::BACKPACK_LV3: return Rarity::Epic;
	case ItemID::BACKPACK_LV4: return Rarity::Legendary;

	case ItemID::THERMITE_GRENADE: return Rarity::Common;
	case ItemID::FRAG_GRENADE: return Rarity::Common;
	case ItemID::ARC_STAR: return Rarity::Common;

	case ItemID::HCOG_CLASSIC: return Rarity::Common;
	case ItemID::HCOG_BRUISER: return Rarity::Rare;
	case ItemID::HOLO: return Rarity::Common;
	case ItemID::VARIABLE_HOLO: return Rarity::Rare;
	case ItemID::DIGITAL_THREAT: return Rarity::Legendary;
	case ItemID::HCOG_RANGER: return Rarity::Epic;
	case ItemID::VARIABLE_AOG: return Rarity::Epic;
	case ItemID::SNIPER: return Rarity::Rare;
	case ItemID::VARIABLE_SNIPER: return Rarity::Epic;
	case ItemID::DIGITAL_SNIPER_THREAT: return Rarity::Legendary;

	case ItemID::BARREL_STABILIZER_LV1: return Rarity::Common;
	case ItemID::BARREL_STABILIZER_LV2: return Rarity::Rare;
	case ItemID::BARREL_STABILIZER_LV3: return Rarity::Epic;
	case ItemID::BARREL_STABILIZER_LV4: return Rarity::Legendary;
	case ItemID::LIGHT_MAGAZINE_LV1: return Rarity::Common;
	case ItemID::LIGHT_MAGAZINE_LV2: return Rarity::Rare;
	case ItemID::LIGHT_MAGAZINE_LV3: return Rarity::Epic;
	case ItemID::HEAVY_MAGAZINE_LV1: return Rarity::Common;
	case ItemID::HEAVY_MAGAZINE_LV2: return Rarity::Rare;
	case ItemID::HEAVY_MAGAZINE_LV3: return Rarity::Epic;
	case ItemID::SNIPER_MAGAZINE_LV1: return Rarity::Common;
	case ItemID::SNIPER_MAGAZINE_LV2: return Rarity::Rare;
	case ItemID::SNIPER_MAGAZINE_LV3: return Rarity::Epic;
	case ItemID::SHOTGUN_BOLT_LV1: return Rarity::Common;
	case ItemID::SHOTGUN_BOLT_LV2: return Rarity::Rare;
	case ItemID::SHOTGUN_BOLT_LV3: return Rarity::Epic;
	case ItemID::STANDARD_STOCK_LV1: return Rarity::Common;
	case ItemID::STANDARD_STOCK_LV2: return Rarity::Rare;
	case ItemID::STANDARD_STOCK_LV3: return Rarity::Epic;
	case ItemID::SNIPER_STOCK_LV1: return Rarity::Common;
	case ItemID::SNIPER_STOCK_LV2: return Rarity::Rare;
	case ItemID::SNIPER_STOCK_LV3: return Rarity::Epic;

	case ItemID::SELECTFIRE_RECEIVER: return Rarity::Epic;
	case ItemID::PRECISION_CHOKE: return Rarity::Epic;
	case ItemID::HAMMERPOINT_ROUNDS: return Rarity::Epic;
	case ItemID::ANVIL_RECEIVER: return Rarity::Legendary;
	case ItemID::DOUBLE_TAP_TRIGGER: return Rarity::Epic;
	case ItemID::VAULT_KEY: return Rarity::Special;

	default: return static_cast<Rarity>(255);
	}
}
ItemSet item_flag(ItemID item) {
	return ItemSet(1) << static_cast<size_t>(item);
}

ItemSet weapon_set(WeaponID weapon) {
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

	switch (weapon) {
		// Energy weapons

		case WeaponID::HAVOC: return
			item_flag(ItemID::ENERGY_AMMO) |
			AR_OPTICS |
			STANDARD_STOCK |
			item_flag(ItemID::SELECTFIRE_RECEIVER);

		case WeaponID::LSTAR: return
			item_flag(ItemID::ENERGY_AMMO) |
			LMG_OPTICS |
			STANDARD_STOCK;

		// Legendary weapons

		case WeaponID::KRABER: return ItemSet();
		case WeaponID::MASTIFF: return ItemSet();
		case WeaponID::DEVOTION: return ItemSet();

		// Sniper weapons

		case WeaponID::SENTINEL: return
			item_flag(ItemID::SNIPER_AMMO) |
			SNIPER_MAGAZINE |
			SNIPER_OPTICS |
			SNIPER_STOCK;

		case WeaponID::CHARGE_RIFLE: return
			item_flag(ItemID::SNIPER_AMMO) |
			SNIPER_OPTICS |
			SNIPER_STOCK;

		case WeaponID::LONGBOW: return
			item_flag(ItemID::SNIPER_AMMO) |
			BARREL_STABILIZER |
			SNIPER_MAGAZINE |
			SNIPER_OPTICS |
			SNIPER_STOCK;

		case WeaponID::TRIPLE_TAKE: return
			item_flag(ItemID::SNIPER_AMMO) |
			SNIPER_MAGAZINE |
			SNIPER_OPTICS |
			SNIPER_STOCK |
			item_flag(ItemID::PRECISION_CHOKE);

		// Heavy weapons

		case WeaponID::WINGMAN: return
			item_flag(ItemID::HEAVY_ROUNDS) |
			HEAVY_MAGAZINE |
			PISTOL_OPTICS;

		case WeaponID::SPITFIRE: return
			item_flag(ItemID::HEAVY_ROUNDS) |
			BARREL_STABILIZER |
			HEAVY_MAGAZINE |
			LMG_OPTICS |
			STANDARD_STOCK;

		case WeaponID::PROWLER: return
			item_flag(ItemID::HEAVY_ROUNDS) |
			HEAVY_MAGAZINE |
			SMG_OPTICS |
			STANDARD_STOCK |
			item_flag(ItemID::SELECTFIRE_RECEIVER);

		case WeaponID::HEMLOCK: return
			item_flag(ItemID::HEAVY_ROUNDS) |
			BARREL_STABILIZER |
			HEAVY_MAGAZINE |
			AR_OPTICS |
			STANDARD_STOCK;

		case WeaponID::FLATLINE: return
			item_flag(ItemID::HEAVY_ROUNDS) |
			HEAVY_MAGAZINE |
			AR_OPTICS |
			STANDARD_STOCK |
			item_flag(ItemID::ANVIL_RECEIVER);

		// Light weapons

		case WeaponID::RE45: return
			item_flag(ItemID::LIGHT_ROUNDS) |
			BARREL_STABILIZER |
			LIGHT_MAGAZINE |
			PISTOL_OPTICS;

		case WeaponID::P2020: return
			item_flag(ItemID::LIGHT_ROUNDS) |
			LIGHT_MAGAZINE |
			PISTOL_OPTICS |
			item_flag(ItemID::HAMMERPOINT_ROUNDS);

		case WeaponID::R301: return
			item_flag(ItemID::LIGHT_ROUNDS) |
			BARREL_STABILIZER |
			LIGHT_MAGAZINE |
			AR_OPTICS |
			STANDARD_STOCK |
			item_flag(ItemID::ANVIL_RECEIVER);

		case WeaponID::R99: return
			item_flag(ItemID::LIGHT_ROUNDS) |
			BARREL_STABILIZER |
			LIGHT_MAGAZINE |
			SMG_OPTICS |
			STANDARD_STOCK;

		case WeaponID::ALTERNATOR: return
			item_flag(ItemID::LIGHT_ROUNDS) |
			BARREL_STABILIZER |
			LIGHT_MAGAZINE |
			SMG_OPTICS |
			STANDARD_STOCK;

		case WeaponID::G7_SCOUT: return
			item_flag(ItemID::LIGHT_ROUNDS) |
			BARREL_STABILIZER |
			LIGHT_MAGAZINE |
			AR_OPTICS |
			STANDARD_STOCK |
			item_flag(ItemID::DOUBLE_TAP_TRIGGER);

		// Shotgun weapons

		case WeaponID::MOZAMBIQUE: return
			item_flag(ItemID::SHOTGUN_SHELLS) |
			SHOTGUN_BOLT |
			SHOTGUN_OPTICS;

		case WeaponID::PEACEKEEPER: return
			item_flag(ItemID::SHOTGUN_SHELLS) |
			SHOTGUN_BOLT |
			SHOTGUN_OPTICS |
			item_flag(ItemID::PRECISION_CHOKE);

		case WeaponID::EVA8_AUTO: return
			item_flag(ItemID::SHOTGUN_SHELLS) |
			SHOTGUN_BOLT |
			SHOTGUN_OPTICS |
			item_flag(ItemID::DOUBLE_TAP_TRIGGER);

		default: return ItemSet();
	}
}
AmmoType weapon_ammo(WeaponID weapon) {
	switch (weapon) {
		case WeaponID::HAVOC: return AmmoType::EnergyAmmo;
		case WeaponID::LSTAR: return AmmoType::EnergyAmmo;

		case WeaponID::KRABER: return AmmoType::SpecialHeavyRounds;
		case WeaponID::MASTIFF: return AmmoType::SpecialShotgunShells;
		case WeaponID::DEVOTION: return AmmoType::ExperimentalAmmo;

		case WeaponID::SENTINEL: return AmmoType::SniperAmmo;
		case WeaponID::CHARGE_RIFLE: return AmmoType::SniperAmmo;
		case WeaponID::LONGBOW: return AmmoType::SniperAmmo;
		case WeaponID::TRIPLE_TAKE: return AmmoType::SniperAmmo;

		case WeaponID::WINGMAN: return AmmoType::HeavyRounds;
		case WeaponID::SPITFIRE: return AmmoType::HeavyRounds;
		case WeaponID::PROWLER: return AmmoType::HeavyRounds;
		case WeaponID::HEMLOCK: return AmmoType::HeavyRounds;
		case WeaponID::FLATLINE: return AmmoType::HeavyRounds;

		case WeaponID::RE45: return AmmoType::LightRounds;
		case WeaponID::P2020: return AmmoType::LightRounds;
		case WeaponID::R301: return AmmoType::LightRounds;
		case WeaponID::R99: return AmmoType::LightRounds;
		case WeaponID::ALTERNATOR: return AmmoType::LightRounds;
		case WeaponID::G7_SCOUT: return AmmoType::LightRounds;

		case WeaponID::MOZAMBIQUE: return AmmoType::ShotgunShells;
		case WeaponID::PEACEKEEPER: return AmmoType::ShotgunShells;
		case WeaponID::EVA8_AUTO: return AmmoType::ShotgunShells;

		default: return static_cast<AmmoType>(255);
	}
}
