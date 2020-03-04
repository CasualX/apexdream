#include "entities.hpp"
#include "data.hpp"
#include "process.hpp"

#include <cmath>

BaseEntity::BaseEntity(uint64_t address) : address(address) {}
PlayerEntity::PlayerEntity(uint64_t address) : BaseEntity(address), bones(new Mat3x4[MAXSTUDIOBONES]) {}
BaseNPCEntity::BaseNPCEntity(uint64_t address) : BaseEntity(address), bones(new Mat3x4[MAXSTUDIOBONES]) {}
WeaponXEntity::WeaponXEntity(uint64_t address) : BaseEntity(address) {}
PropSurvivalEntity::PropSurvivalEntity(uint64_t address) : BaseEntity(address) {}
PlayerResourceEntity::PlayerResourceEntity(uint64_t address) : BaseEntity(address), name_pointers(new uint64_t[64]), names(new std::string[64]()) {}
WorldEntity::WorldEntity(uint64_t address) : BaseEntity(address) {}

//----------------------------------------------------------------

void PlayerEntity::update(const GameProcess& process, const GameData& data) {
	FloatInt temp[20];

	process.read(address + 0x8, handle);
	process.read(address + data.entity_flags, flags);
	process.read(address + data.entity_velocity, velocity);

	if (process.read_array(address + data.entity_origin, temp, 9)) {
		origin = Vec3{temp[0].f32, temp[1].f32, temp[2].f32};
		angles = Vec3{temp[6].f32, temp[7].f32, temp[8].f32};
	}

	process.read(address + data.player_max_health, max_health);
	process.read(address + data.player_health, health);
	process.read(address + data.entity_team_num, team_num);
	process.read(address + data.player_life_state, life_state);
	process.read(address + data.player_bleedout_state, bleedout_state);

	if (process.read_array(address + data.player_shields, temp, 2)) {
		shields = temp[0].i32;
		max_shields = temp[1].i32;
	}
	if (process.read_array(address + data.player_helmet_armor_type, temp, 2)) {
		helmet_type = temp[0].i32;
		armor_type = temp[1].i32;
	}

	if (process.read_array(address + data.player_latest_primary_weapons, temp, 2)) {
		latest_primary_weapons[0] = EHandle{temp[0].u32};
		latest_primary_weapons[1] = EHandle{temp[1].u32};
	}

	uint64_t model_name_ptr;
	if (process.read(address + data.entity_model_name, model_name_ptr)) {
		char model_buffer[128];
		if (process.read(model_name_ptr, model_buffer)) {
			model_name.assign(&model_buffer[0]);
		}
	}

	uint64_t bones_ptr;
	if (process.read(address + data.entity_bones, bones_ptr)) {
		process.read_array(bones_ptr, bones.get(), MAXSTUDIOBONES);
	}

	if (process.read_array(address + data.player_observer_mode, temp, 2)) {
		observer_mode = temp[0].i32;
		observer_target = EHandle{temp[1].u32};
	}

	if (process.read_array(address + data.player_zoom_state, temp, 5)) {
		zooming = (temp[0].u32 & 0x0000ff00) != 0;
		zoom_base_frac = temp[1].f32;
		zoom_base_time = temp[2].f32;
		zoom_full_start_time = temp[3].f32;
	}

	if (process.read_array(address + data.player_camera_data, temp, 6)) {
		camera_origin = Vec3{temp[0].f32, temp[1].f32, temp[2].f32};
		camera_angles = Vec3{temp[3].f32, temp[4].f32, temp[5].f32};
	}
}
Vec3 PlayerEntity::get_bone_pos(size_t bone) const {
	if (bone < MAXSTUDIOBONES) {
		const auto& mat = bones[bone];
		return origin + Vec3{mat.a[3], mat.a[7], mat.a[11]};
	}
	else {
		return origin;
	}
}
ItemSet PlayerEntity::get_desired_items() const {
	// TODO! Learn if this player already has these items in their inventory
	ItemSet set =
		item_flag(ItemID::KNOCKDOWN_SHIELD_LV3) |
		item_flag(ItemID::KNOCKDOWN_SHIELD_LV4) |
		item_flag(ItemID::BACKPACK_LV2) |
		item_flag(ItemID::BACKPACK_LV3) |
		item_flag(ItemID::BACKPACK_LV4);

	bool needs_health = health < max_health;
	bool needs_shields = shields < max_shields;

	// Healing items
	if (needs_health) {
		set |= item_flag(ItemID::SYRINGE);
	}
	if (needs_shields) {
		set |= item_flag(ItemID::SHIELD_CELL);
	}
	if (needs_health && needs_shields) {
		set |= item_flag(ItemID::PHOENIX_KIT);
	}
	set |= item_flag(ItemID::MED_KIT);
	set |= item_flag(ItemID::SHIELD_BATTERY);

	// Helmets
	if (helmet_type < 1) {
		set |= item_flag(ItemID::HELMET_LV1);
	}
	if (helmet_type < 2) {
		set |= item_flag(ItemID::HELMET_LV2);
	}
	if (helmet_type < 3) {
		set |= item_flag(ItemID::HELMET_LV3);
	}
	if (helmet_type <= 4) {
		set |= item_flag(ItemID::HELMET_LV4);
	}

	// Body armors
	if (armor_type < 1 || armor_type == 1 && needs_shields) {
		set |= item_flag(ItemID::BODY_ARMOR_LV1);
	}
	if (armor_type < 2 || armor_type == 2 && needs_shields) {
		set |= item_flag(ItemID::BODY_ARMOR_LV2);
	}
	set |= item_flag(ItemID::BODY_ARMOR_LV3) | item_flag(ItemID::BODY_ARMOR_LV4);
	set |= item_flag(ItemID::EVO_SHIELD_LV1) | item_flag(ItemID::EVO_SHIELD_LV2) | item_flag(ItemID::EVO_SHIELD_LV3) | item_flag(ItemID::EVO_SHIELD_LV4);

	return set;
}

//----------------------------------------------------------------

void BaseNPCEntity::update(const GameProcess& process, const GameData& data) {
	FloatInt temp[10];

	process.read(address + 0x8, handle);

	if (process.read_array(address + data.entity_origin, temp, 9)) {
		origin = Vec3{temp[0].f32, temp[1].f32, temp[2].f32};
		angles = Vec3{temp[6].f32, temp[7].f32, temp[8].f32};
	}

	uint64_t model_name_ptr;
	if (process.read(address + data.entity_model_name, model_name_ptr)) {
		char model_buffer[128];
		if (process.read(model_name_ptr, model_buffer)) {
			model_name.assign(&model_buffer[0]);
		}
	}

	uint64_t bones_ptr;
	if (process.read(address + data.entity_bones, bones_ptr)) {
		process.read_array(bones_ptr, bones.get(), MAXSTUDIOBONES);
	}
}
Vec3 BaseNPCEntity::get_bone_pos(size_t bone) const {
	if (bone < MAXSTUDIOBONES) {
		const auto& mat = bones[bone];
		return origin + Vec3{mat.a[3], mat.a[7], mat.a[11]};
	}
	else {
		return origin;
	}
}

//----------------------------------------------------------------

void WeaponXEntity::update(const GameProcess& process, const GameData& data) {
	FloatInt temp[8];

	process.read(address + 0x8, handle);
	process.read(address + data.weaponx_weapon_owner, weapon_owner);
	process.read(address + data.weaponx_weapon_name_index, weapon_name_index);

	if (process.read_array(address + data.weaponx_player_data_zoom_fov, temp, 2)) {
		cur_zoom_fov = temp[0].f32;
		target_zoom_fov = temp[1].f32;
	}

	if (process.read_array(address + data.weaponx_projectile_speed, temp, 4)) {
		projectile_speed = temp[0].f32;
		projectile_scale = temp[2].f32;
	}
}
float WeaponXEntity::get_projectile_speed() const {
	return projectile_speed;
}
float WeaponXEntity::get_projectile_gravity() const {
	return /*sv_gravity*/750.0f * projectile_scale;
}
ItemSet WeaponXEntity::get_desired_items() const {
	// TODO! Filter items which are worse than the attachments we already have.
	return weapon_set(weapon_name_index);
}

//----------------------------------------------------------------

void PropSurvivalEntity::update(const GameProcess& process, const GameData& data) {
	process.read(address + 0x8, handle);
	process.read(address + data.entity_origin, origin);

	FloatInt temp[5];
	if (process.read_array(address + data.prop_survival, temp, 5)) {
		ammo_in_clip = temp[0].i32;
		custom_script_int = static_cast<ItemID>(temp[1].u32);
		survival_property = temp[2].u32;
		weapon_name_index = static_cast<WeaponID>(temp[3].u32);
		mod_bit_field = temp[4].u32;
	}
}

//----------------------------------------------------------------

void PlayerResourceEntity::update(const GameProcess& process, const GameData& data) {
	process.read(address + 0x8, handle);

	uint64_t new_name_pointers[64];
	char buffer[256];
	if (process.read_array(address + data.player_resource_names, new_name_pointers, 64)) {
		for (size_t i = 0; i < 64; i += 1) {
			if (name_pointers[i] != new_name_pointers[i]) {
				if (process.read(new_name_pointers[i], buffer)) {
					names[i].assign(buffer);
				}
			}
		}
		memcpy(name_pointers.get(), new_name_pointers, sizeof(new_name_pointers));
	}
}
const char* PlayerResourceEntity::get_name(size_t index) const {
	return index < 64 ? names[index].c_str() : nullptr;
}

//----------------------------------------------------------------

void WorldEntity::update(const GameProcess& process, const GameData& data) {
	FloatInt temp[10];
	if (process.read_array(address + data.world_death_field, temp, 7)) {
		death_field_is_active = temp[0].bytes[0] != 0;
		death_field_origin = Vec3{temp[1].f32, temp[2].f32, temp[3].f32};
		death_field_radius_start = temp[4].f32;
		death_field_radius_end = temp[5].f32;
		death_field_time_start = temp[6].f32;
		death_field_time_end = temp[7].f32;
	}
}
float WorldEntity::death_field_radius(float curtime) const {
	if (death_field_time_start == death_field_time_end) {
		return 0.0f;
	}
	const float fraction = fmin(fmax((curtime - death_field_time_start) / (death_field_time_end - death_field_time_start), 0.0f), 1.0f);
	return death_field_radius_start + fraction * (death_field_radius_end - death_field_radius_start);
}
