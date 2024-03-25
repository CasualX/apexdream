use super::*;

#[derive(Default)]
pub struct WeaponXEntity {
    pub entity_ptr: sdk::Ptr,
    pub entity_size: u32,
    pub index: u32,

    update_rate: u32,

    pub weapon_owner: sdk::EHandle,

    pub mod_bitfield: u32,

    pub last_primary_attack_time: f32,
    pub next_ready_time: f32,
    pub next_primary_attack_time: f32,
    pub attack_time_this_frame: f32,

    pub ammo_in_clip: i32,
    pub ammo_in_stockpile: i32,
    pub lifetime_shots: i32,
    pub time_weapon_idle: f32,
    pub weap_state: sdk::WeapState,
    pub discarded: bool,
    pub in_reload: bool,

    // Charging state
    // Havoc, L-Star: while firing
    // Triple Take, Peacekeeper: choke
    // Bocek: while drawing
    pub weapon_is_charging: bool,
    pub charge_end_time: f32,
    pub charge_start_time: f32,
    pub last_charge_level: i32,

    pub burst_fire_count: i32,
    pub burst_fire_index: i32,
    pub shot_index: i32,
    pub shot_count: i32,

    pub cur_zoom_fov: f32,
    pub target_zoom_fov: f32,

    pub weapon_name_index: i32,
    pub weapon_name: sdk::WeaponName,
    pub modifiers_ptr: sdk::Ptr,

    pub is_semi_auto: bool,
    pub projectile_scale: f32,
    pub projectile_speed: f32,
}
impl WeaponXEntity {
    pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
        let entity_size = cc.ClassSize;
        Box::new(WeaponXEntity {
            entity_ptr,
            entity_size,
            index,
            ..WeaponXEntity::default()
        }) as Box<dyn Entity>
    }

    pub fn is_mod_enabled(&self, state: &GameState, mod_name: sdk::ModName) -> bool {
        let flag = state.get_mod(self.weapon_name, mod_name);
        self.mod_bitfield & flag != 0
    }

    /// Returns the set of desired items for this weapon
    pub fn desired_items(&self, state: &GameState) -> sdk::ItemSet {
        let Some(mods) = state.get_mods(self.weapon_name) else {
            return Default::default();
        };

        let mut desired = sdk::ItemSet::default();
        let ammo_type_index = sdk::ammo_type(self.weapon_name).0 as usize;
        if ammo_type_index < desired.bit_len() {
            desired.bit_set(ammo_type_index);
        }
        for (index, mod_name) in mods.iter().enumerate() {
            let mod_name = sdk::ModName(hash(mod_name));

            let is_set = self.mod_bitfield & (1 << index) != 0;

            if mod_name == sdk::ModName::alt_ammo {
                desired.bit_set(sdk::ItemId::LightRounds.0 as usize);
                desired.bit_set(sdk::ItemId::HeavyRounds.0 as usize);
            }

            if mod_name == sdk::ModName::crate_ && is_set {
                break;
            }

            let item = sdk::mod_name_item(mod_name);
            if item != sdk::ItemId::None {
                let item_index = item.0 as usize;
                if item_index < desired.bit_len() {
                    desired.bit_set(item_index);
                }

                if is_set {
                    desired.bit_andnot(&sdk::downgrade_mask(item));
                }
            }
        }
        desired
    }
    pub fn ammo_item(&self, state: &GameState) -> sdk::ItemId {
        if matches!(self.weapon_name, sdk::WeaponName::CAR) {
            if self.is_mod_enabled(state, sdk::ModName::alt_ammo) {
                return sdk::ItemId::LightRounds;
            }
        }
        return sdk::ammo_type(self.weapon_name);
    }
}
impl Entity for WeaponXEntity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_ref(&self) -> EntityRef<'_> {
        EntityRef::WeaponX(self)
    }
    fn is_serialized(&self) -> bool {
        true
    }
    fn get_info(&self) -> EntityInfo {
        EntityInfo {
            entity_ptr: self.entity_ptr,
            index: self.index as usize,
            handle: sdk::EHandle::from(self.index),
            rate: self.update_rate,
        }
    }
    fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
        #[derive(sdk::Pod)]
        #[repr(C)]
        struct Indices {
            weapon_owner: u32,
            attack_time: [u32; 4],
            ammo: [u32; 6],
            zoom_fov: [u32; 2],
            charge: [u32; 4],
            burst: [u32; 4],
            weapon_name_index: [u32; 3],
            mod_bitfield: [u32; 3],
            data: [u32; 1],
            projectile: [u32; 2],
        }

        let data = ctx.data;
        let mut indices = Indices {
            weapon_owner: data.weaponx_weapon_owner,
            attack_time: [
                data.weaponx_next_primary_attack - 8,
                data.weaponx_next_primary_attack - 4,
                data.weaponx_next_primary_attack,
                data.weaponx_next_primary_attack + 4,
            ],
            ammo: [
                data.weaponx_ammo_in_clip + 0,
                data.weaponx_ammo_in_clip + 4,
                data.weaponx_ammo_in_clip + 8,
                data.weaponx_ammo_in_clip + 12,
                data.weaponx_ammo_in_clip + 16,
                data.weaponx_ammo_in_clip + 20,
            ],
            zoom_fov: [
                data.weaponx_player_data + data.weaponx_zoom_fov + 0,
                data.weaponx_player_data + data.weaponx_zoom_fov + 4,
            ],
            charge: [
                data.weaponx_charge_start_time + 0,
                data.weaponx_charge_start_time + 4,
                data.weaponx_charge_start_time + 20,
                data.weaponx_charge_start_time + 24,
            ],
            burst: [
                data.weaponx_burst_fire + 0,
                data.weaponx_burst_fire + 4,
                data.weaponx_burst_fire + 8,
                data.weaponx_burst_fire + 12,
            ],
            weapon_name_index: [
                data.weaponx_weapon_name_index,
                data.weaponx_weapon_name_index + 0x20,
                data.weaponx_weapon_name_index + 0x24,
            ],
            mod_bitfield: [
                data.weaponx_mod_bitfield + 0,
                data.weaponx_mod_bitfield + 4,
                data.weaponx_mod_bitfield + 8,
            ],
            data: [data.weaponx_is_semi_auto & !3],
            projectile: [
                data.weaponx_projectile_speed + 0,
                data.weaponx_projectile_speed + 8,
            ],
        };

        if let Ok(fields) = api.vm_gatherd(self.entity_ptr, self.entity_size, &mut indices) {
            self.weapon_owner = sdk::EHandle::from(fields.weapon_owner);

            self.last_primary_attack_time = f32::from_bits(fields.attack_time[0]);
            self.next_ready_time = f32::from_bits(fields.attack_time[1]);
            self.next_primary_attack_time = f32::from_bits(fields.attack_time[2]);
            self.attack_time_this_frame = f32::from_bits(fields.attack_time[3]);

            self.ammo_in_clip = fields.ammo[0] as i32;
            self.ammo_in_stockpile = fields.ammo[1] as i32;
            self.lifetime_shots = fields.ammo[2] as i32;
            self.time_weapon_idle = f32::from_bits(fields.ammo[3] as u32);
            self.weap_state = sdk::WeapState(fields.ammo[4] as i32);
            self.discarded = fields.ammo[5].to_le_bytes()[1] != 0;
            self.in_reload = fields.ammo[5].to_le_bytes()[2] != 0;

            self.cur_zoom_fov = f32::from_bits(fields.zoom_fov[0]);
            self.target_zoom_fov = f32::from_bits(fields.zoom_fov[1]);

            self.charge_start_time = f32::from_bits(fields.charge[0]);
            self.charge_end_time = f32::from_bits(fields.charge[1]);
            self.weapon_is_charging = fields.charge[2] & 0x0000ff00 != 0;
            self.last_charge_level = fields.charge[3] as i32;

            self.burst_fire_count = fields.burst[0] as i32;
            self.burst_fire_index = fields.burst[1] as i32;
            self.shot_index = fields.burst[2] as i32;
            self.shot_count = fields.burst[3] as i32;

            self.weapon_name_index = fields.weapon_name_index[0] as u16 as i32;

            self.modifiers_ptr = sdk::Ptr::from_raw(
                fields.weapon_name_index[1] as u64 | (fields.weapon_name_index[2] as u64) << 32,
            );

            self.mod_bitfield =
                fields.mod_bitfield[0] | fields.mod_bitfield[1] | fields.mod_bitfield[2];

            self.is_semi_auto =
                fields.data[0].to_le_bytes()[data.weaponx_is_semi_auto as usize & 3] != 0;
            self.projectile_speed = f32::from_bits(fields.projectile[0]);
            self.projectile_scale = f32::from_bits(fields.projectile[1]);
        }

        self.update_rate = if self.weapon_owner == ctx.local_entity {
            1
        } else {
            512
        };
    }
    fn post(&mut self, _api: &mut Api, _ctx: &UpdateContext, state: &GameState) {
        self.weapon_name = state.weapon_name(self.weapon_name_index);
    }
}
impl crate::base::solver::ProjectileWeapon for WeaponXEntity {
    fn projectile_speed(&self) -> f32 {
        if self.weapon_name == sdk::WeaponName::BOCEK {
            return if self.projectile_scale == 1.5 {
                28000.0
            } else {
                10000.0
            };
        }
        self.projectile_speed
    }
    fn projectile_gravity(&self) -> f32 {
        /*sv_gravity*/
        750.0 * self.projectile_scale
    }
    fn projectile_collection(&self) -> Option<crate::base::solver::Collection> {
        use sdk::WeaponName as W;
        match self.weapon_name {
            W::SENTINEL => Some(sdk::projectiles::SENTINEL),
            W::KRABER => Some(sdk::projectiles::KRABER),
            W::BOCEK => Some(sdk::projectiles::BOCEK),
            W::R301 => Some(sdk::projectiles::R301),
            W::G7_SCOUT => Some(sdk::projectiles::G7_SCOUT),
            W::REPEATER => Some(sdk::projectiles::REPEATER),
            W::LONGBOW => Some(sdk::projectiles::LONGBOW),
            W::FLATLINE => Some(sdk::projectiles::FLATLINE),
            W::WINGMAN => Some(sdk::projectiles::WINGMAN),
            W::PROWLER => Some(sdk::projectiles::PROWLER),
            W::VOLT => Some(sdk::projectiles::VOLT),
            W::DEVOTION => Some(sdk::projectiles::DEVOTION),
            W::SPITFIRE => Some(sdk::projectiles::SPITFIRE),
            W::HEMLOK => Some(sdk::projectiles::HEMLOK),
            W::THROWING_KNIFE => Some(sdk::projectiles::THROWING_KNIFE),
            _ => None,
        }
    }
}
