use super::*;

const MAX_ITEMS: usize = 400;

#[derive(Default)]
pub struct LootItems {
	models: Vec<String>,
	table: Vec<sdk::ItemId>,
	visualize: bool,
}

impl LootItems {
	pub fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
		// Reload the itemids on new connection
		if ctx.connected {
			self.models.resize_with(MAX_ITEMS, String::new);
			self.table.resize(MAX_ITEMS, sdk::ItemId::None);
		}

		// Visualize the cache if successfully identified an item
		if self.visualize {
			self.visualize = false;
			api.visualize(s!("Items"), xfmt! {
				<pre>
					for index in 0..MAX_ITEMS {
						if let (Some(&known), Some(model)) = (self.table.get(index), self.models.get(index)) {
							if known != sdk::ItemId::None {
								{index}": "{model}"\n"
							}
						}
					}
				</pre>
			});
		}
	}

	pub fn visit(&mut self, _api: &mut Api, _ctx: &UpdateContext, entity_ref: EntityRef<'_>) {
		let loot = match entity_ref {
			EntityRef::Loot(loot) => loot,
			_ => return,
		};

		let new_ki = analyze(loot);
		if new_ki == sdk::ItemId::None {
			return;
		}

		let index = loot.custom_script_int as usize;
		let Some(p_ki) = self.table.get_mut(index) else { return };
		let Some(p_model) = self.models.get_mut(index) else { return };

		self.visualize |= *p_ki != new_ki;
		*p_ki = new_ki;

		if loot.model_name.string.len() != 0 {
			p_model.clone_from(&loot.model_name.string);
		}
	}
}

impl GameState {
	pub fn known_item(&self, custom_script_int: i32) -> sdk::ItemId {
		self.items.table.get(custom_script_int as usize).cloned().unwrap_or(sdk::ItemId::None)
	}
}

fn analyze(loot: &LootEntity) -> sdk::ItemId {
	use sdk::WeaponName as WN;
	use sdk::ModelName as MN;
	use sdk::ItemId as KI;

	if loot.weapon_name.0 != 0 {
		match loot.weapon_name {
			WN::R301 => KI::R301,
			WN::SENTINEL => KI::Sentinel,
			WN::BOCEK => KI::Bocek,
			WN::ALTERNATOR => KI::Alternator,
			WN::RE45 => KI::RE45,
			WN::CHARGE_RIFLE => KI::ChargeRifle,
			WN::DEVOTION => KI::Devotion,
			WN::LONGBOW => KI::Longbow,
			WN::HAVOC => KI::Havoc,
			WN::EVA8_AUTO => KI::EVA8Auto,
			WN::FLATLINE => KI::Flatline,
			WN::HEMLOK => KI::Hemlok,
			WN::KRABER => KI::Kraber,
			WN::G7_SCOUT => KI::G7Scout,
			WN::LSTAR => KI::LStar,
			WN::MASTIFF => KI::Mastiff,
			WN::MOZAMBIQUE => KI::Mozambique,
			WN::PROWLER => KI::Prowler,
			WN::PEACEKEEPER => KI::PK,
			WN::R99 => KI::R99,
			WN::P2020 => KI::P2020,
			WN::SPITFIRE => KI::Spitfire,
			WN::TRIPLE_TAKE => KI::TripleTake,
			WN::WINGMAN => KI::Wingman,
			WN::VOLT => KI::Volt,
			WN::REPEATER => KI::Repeater,
			WN::RAMPAGE => KI::Rampage,
			WN::CAR => KI::CAR,
			_ => KI::None,
		}
	}
	else if loot.custom_script_int < 5 {
		KI::None
	}
	else {
		match loot.model_name.hash {
			MN::LIGHT_ROUNDS => KI::LightRounds,
			MN::ENERGY_AMMO => KI::EnergyAmmo,
			MN::SHOTGUN_AMMO => KI::ShotgunShells,
			MN::HEAVY_ROUNDS => KI::HeavyRounds,
			MN::SNIPER_AMMO => KI::SniperAmmo,
			MN::ARROWS | MN::ARROWS_SINGLE => KI::Arrows,

			MN::ULTIMATE_ACCELERANT => KI::UltAccel,
			MN::PHOENIX_KIT => KI::PhoenixKit,
			MN::MED_KIT => KI::MedKit,
			MN::SYRINGE => KI::Syringe,
			MN::SHIELD_BATTERY => KI::Battery,
			MN::SHIELD_CELL => KI::ShieldCell,

			MN::THERMITE_GRENADE => KI::Thermite,
			MN::FRAG_GRENADE => KI::FragGrenade,
			MN::ARC_STAR => KI::ArcStar,

			MN::UPGRADE_HEAD => from_color(&loot.color, &[KI::HelmetLv1, KI::HelmetLv2, KI::HelmetLv3, KI::HelmetLv4, KI::None]),
			MN::UPGRADE_BODY => from_color(&loot.color, &[KI::EvoShieldLv1, KI::EvoShieldLv2, KI::EvoShieldLv3, KI::BodyArmorLv4, KI::EvoShieldLv4]),
			MN::KNOCKDOWN_SHIELD => from_color(&loot.color, &[KI::KnockdownShieldLv1, KI::KnockdownShieldLv2, KI::KnockdownShieldLv3, KI::KnockdownShieldLv4, KI::None]),
			MN::BACKPACK_LIGHT => KI::BackpackLv1,
			MN::BACKPACK_MEDIUM => KI::BackpackLv2,
			MN::BACKPACK_HEAVY => from_color(&loot.color, &[KI::None, KI::None, KI::BackpackLv3, KI::BackpackLv4, KI::None]),

			MN::HCOG_CLASSIC => KI::HcogClassic,
			MN::HCOG_BRUISER => KI::HcogBruiser,
			MN::HCOG_RANGER => KI::HcogRanger,
			MN::DIGITAL_THREAT => KI::DigiThreat,
			MN::HOLO => KI::Holo,
			MN::VARIABLE_HOLO => KI::VariableHolo,
			MN::VARIABLE_AOG => KI::VariableAOG,
			MN::SNIPER => KI::Sniper,
			MN::VARIABLE_SNIPER => KI::VariableSniper,
			MN::DIGITAL_SNIPER_THREAT => KI::DigiSniperThreat,

			MN::BARREL_STABILIZER => from_color(&loot.color, &[KI::BarrelStabilizerLv1, KI::BarrelStabilizerLv2, KI::BarrelStabilizerLv3, KI::BarrelStabilizerLv4, KI::None]),
			MN::LASER_SIGHT => from_color(&loot.color, &[KI::LaserSightLv1, KI::LaserSightLv2, KI::LaserSightLv3, KI::LaserSightLv4, KI::None]),
			MN::LIGHT_MAGAZINE => from_color(&loot.color, &[KI::LightMagazineLv1, KI::LightMagazineLv2, KI::LightMagazineLv3, KI::LightMagazineLv4, KI::None]),
			MN::HEAVY_MAGAZINE => from_color(&loot.color, &[KI::HeavyMagazineLv1, KI::HeavyMagazineLv2, KI::HeavyMagazineLv3, KI::HeavyMagazineLv4, KI::None]),
			MN::ENERGY_MAGAZINE => from_color(&loot.color, &[KI::EnergyMagazineLv1, KI::EnergyMagazineLv2, KI::EnergyMagazineLv3, KI::EnergyMagazineLv4, KI::None]),
			MN::SNIPER_MAGAZINE => from_color(&loot.color, &[KI::SniperMagazineLv1, KI::SniperMagazineLv2, KI::SniperMagazineLv3, KI::SniperMagazineLv4, KI::None]),
			MN::SHOTGUN_BOLT => from_color(&loot.color, &[KI::ShotgunBoltLv1, KI::ShotgunBoltLv2, KI::ShotgunBoltLv3, KI::ShotgunBoltLv4, KI::None]),
			MN::STANDARD_STOCK => from_color(&loot.color, &[KI::StandardStockLv1, KI::StandardStockLv2, KI::StandardStockLv3, KI::None, KI::None]),
			MN::SNIPER_STOCK => from_color(&loot.color, &[KI::SniperStockLv1, KI::SniperStockLv2, KI::SniperStockLv3, KI::None, KI::None]),

			MN::MODS_CHIP => {
				match (loot.bits.inside_opacity, loot.skin) {
					(0x60, 0) => KI::LegendaryHopUp0, //
					(0x60, 4) => KI::LegendaryHopUp4, // Turbocharger, Skullpiercer
					(0x5f, 0) => KI::EpicHopUp0, // Deadeye's tempo, Kinetic feeder, Shatter caps, Boosted Loader
					(0x5f, 3) => KI::EpicHopUp3, // Hammerpoint rounds, Double tap trigger
					_ => KI::None,
				}
			},

			MN::TREASURE_BOX => KI::TreasurePack,
			MN::KEYCARD_V1 => KI::Keycard,
			MN::VOID_RING => KI::HeatShield,
			MN::BEACON_CAPSULE => KI::MobileRespawn,
			MN::GOLDEN_TICKET => KI::GoldenTicket,
			MN::BANNER_CRAFTING => KI::BannerCrafting,

			_ => KI::None,
		}
	}
}

fn from_color(color: &[f32; 3], items: &[sdk::ItemId; 5]) -> sdk::ItemId {
	let table = [
		&sdk::COMMON_COLORS,
		&sdk::RARE_COLORS,
		&sdk::EPIC_COLORS,
		&sdk::LEGENDARY_COLORS,
		&sdk::HEIRLOOM_COLORS,
	];
	for i in 0..5 {
		let colors = &table[i];
		for j in 0..colors.len() {
			if color == &colors[j] {
				return items[i];
			}
		}
	}
	return sdk::ItemId::None;
}
