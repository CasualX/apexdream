#include "entities.hpp"
#include "data.hpp"
#include "process.hpp"

#include <cmath>

BaseEntity::BaseEntity(uint64_t address) : address(address) {}
PlayerEntity::PlayerEntity(uint64_t address) : BaseEntity(address), bones(new Mat3x4[MAXSTUDIOBONES]) {}
BaseNPCEntity::BaseNPCEntity(uint64_t address) : BaseEntity(address), bones(new Mat3x4[MAXSTUDIOBONES]) {}
WeaponXEntity::WeaponXEntity(uint64_t address) : BaseEntity(address) {}
PropSurvivalEntity::PropSurvivalEntity(uint64_t address) : BaseEntity(address) {}
WorldEntity::WorldEntity(uint64_t address) : BaseEntity(address) {}

//----------------------------------------------------------------

void PlayerEntity::update(const GameProcess& process) {
	FloatInt temp[20];

	process.read(address + 0x8, handle);
	process.read(address + data::ENTITY_FLAGS, flags);
	process.read(address + data::ENTITY_VELOCITY, velocity);

	if (process.read_array(address + data::ENTITY_ORIGIN, temp, 9)) {
		origin = Vec3{temp[0].f32, temp[1].f32, temp[2].f32};
		angles = Vec3{temp[6].f32, temp[7].f32, temp[8].f32};
	}

	process.read(address + data::ENTITY_MAX_HEALTH, max_health);
	process.read(address + data::ENTITY_HEALTH, health);
	process.read(address + data::ENTITY_TEAM_NUM, team_num);
	process.read(address + data::ENTITY_LIFE_STATE, life_state);
	process.read(address + data::PLAYER_BLEEDOUT_STATE, bleedout_state);

	if (process.read_array(address + data::ENTITY_SHIELDS, temp, 2)) {
		shields = temp[0].i32;
		max_shields = temp[1].i32;
	}
	if (process.read_array(address + data::PLAYER_HELMET_ARMOR_TYPE, temp, 2)) {
		helmet_type = temp[0].i32;
		armor_type = temp[1].i32;
	}

	if (process.read_array(address + data::BCC_LATEST_PRIMARY_WEAPONS, temp, 2)) {
		latest_primary_weapons[0] = EHandle{temp[0].u32};
		latest_primary_weapons[1] = EHandle{temp[1].u32};
	}

	process.read(address + data::BCC_LAST_VISIBLE_TIME, last_visible_time);

	uint64_t model_name_ptr;
	if (process.read(address + data::ENTITY_MODEL_NAME, model_name_ptr)) {
		char model_buffer[128];
		if (process.read(model_name_ptr, model_buffer)) {
			model_name.assign(&model_buffer[0]);
		}
	}

	uint64_t bones_ptr;
	if (process.read(address + data::ANIMATING_BONE_ARRAY, bones_ptr)) {
		process.read_array(bones_ptr, bones.get(), MAXSTUDIOBONES);
	}

	if (process.read_array(address + data::PLAYER_OBSERVER_MODE, temp, 2)) {
		observer_mode = temp[0].i32;
		observer_target = EHandle{temp[1].u32};
	}

	if (process.read_array(address + data::PLAYER_ZOOM_STATE, temp, 5)) {
		zooming = (temp[0].u32 & 0x0000ff00) != 0;
		zoom_base_frac = temp[1].f32;
		zoom_base_time = temp[2].f32;
		zoom_full_start_time = temp[3].f32;
	}

	if (process.read_array(address + data::PLAYER_CAMERA_DATA, temp, 6)) {
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
bool PlayerEntity::is_visible(float curtime) const {
	return last_visible_time > 0.0f && fabsf(last_visible_time - curtime) < 0.1f;
}

//----------------------------------------------------------------

void BaseNPCEntity::update(const GameProcess& process) {
	FloatInt temp[10];

	process.read(address + 0x8, handle);

	if (process.read_array(address + data::ENTITY_ORIGIN, temp, 9)) {
		origin = Vec3{temp[0].f32, temp[1].f32, temp[2].f32};
		angles = Vec3{temp[6].f32, temp[7].f32, temp[8].f32};
	}

	uint64_t model_name_ptr;
	if (process.read(address + data::ENTITY_MODEL_NAME, model_name_ptr)) {
		char model_buffer[128];
		if (process.read(model_name_ptr, model_buffer)) {
			model_name.assign(&model_buffer[0]);
		}
	}

	uint64_t bones_ptr;
	if (process.read(address + data::ANIMATING_BONE_ARRAY, bones_ptr)) {
		process.read_array(bones_ptr, bones.get(), MAXSTUDIOBONES);
	}

	process.read(address + data::BCC_LAST_VISIBLE_TIME, last_visible_time);
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
bool BaseNPCEntity::is_visible(float curtime) const {
	return last_visible_time > 0.0f && fabsf(last_visible_time - curtime) < 0.1f;
}

//----------------------------------------------------------------

void WeaponXEntity::update(const GameProcess& process) {
	FloatInt temp[8];

	process.read(address + 0x8, handle);
	process.read(address + data::WEAPONX_WEAPON_OWNER, weapon_owner);
	process.read(address + data::WEAPONX_WEAPON_NAME_INDEX, weapon_index);

	if (process.read_array(address + data::WEAPONX_AMMO_IN_CLIP, temp, 6)) {
		ammo_in_clip = temp[0].i32;
		ammo_in_stockpile = temp[1].i32;
		lifetime_shots = temp[2].i32;
		time_weapon_idle = temp[3].f32;
		weap_state = temp[4].i32;
		discarded = temp[5].bytes[1];
		in_reload = temp[5].bytes[2];
	}

	if (process.read_array(address + data::WEAPONX_PLAYER_DATA_ZOOM_FOV, temp, 2)) {
		cur_zoom_fov = temp[0].f32;
		target_zoom_fov = temp[1].f32;
	}

	if (process.read_array(address + data::WEAPONX_PROJECTILE_SPEED, temp, 4)) {
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

//----------------------------------------------------------------

void PropSurvivalEntity::update(const GameProcess& process) {
	process.read(address + 0x8, handle);
	process.read(address + data::ENTITY_ORIGIN, origin);

	FloatInt temp[5];
	if (process.read_array(address + data::PROP_SURVIVAL, temp, 5)) {
		ammo_in_clip = temp[0].i32;
		custom_script_int = static_cast<ItemID>(temp[1].u32);
		survival_property = temp[2].u32;
		weapon_index = static_cast<WeaponIndex>(temp[3].u32);
		mod_bit_field = temp[4].u32;
	}
}

//----------------------------------------------------------------

void WorldEntity::update(const GameProcess& process) {
	FloatInt temp[10];
	if (process.read_array(address + data::WORLD_DEATH_FIELD, temp, 7)) {
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
