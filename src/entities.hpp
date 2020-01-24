#pragma once

#include "sdk.hpp"
#include "items.hpp"
#include "solver.hpp"

#include <string>
#include <memory>

class GameProcess;
class GameData;

class BaseEntity {
public:
	explicit BaseEntity(uint64_t address);
	virtual ~BaseEntity() = default;
	virtual void update(const GameProcess& process, const GameData& data) = 0;
public:
	uint64_t address;
	EHandle handle;
};

class PlayerEntity : public BaseEntity {
public:
	explicit PlayerEntity(uint64_t entity_ptr);

	void update(const GameProcess& process, const GameData& data) override;

	inline bool is_onground() const {
		return (flags & 0x1) != 0;
	}
	inline bool is_ducking() const {
		return (flags & 0x2) != 0;
	}
	inline bool is_alive() const {
		return life_state == 0;
	}
	inline bool is_downed() const {
		return bleedout_state != 0;
	}
	inline EHandle active_weapon() const {
		return latest_primary_weapons[0];
	}
	inline Vec3 get_bone_pos(size_t bone) const {
		if (bone < MAXSTUDIOBONES) {
			const auto& mat = bones[bone];
			return origin + Vec3{mat.a[3], mat.a[7], mat.a[11]};
		}
		else {
			return origin;
		}
	}

public:
	Vec3 origin;
	Vec3 angles;
	Vec3 velocity;

	int32_t health, max_health;
	int32_t shields, max_shields;
	int32_t helmet_type, armor_type;

	int32_t team_num;
	uint32_t flags;
	uint8_t life_state;
	int32_t bleedout_state;

	EHandle latest_primary_weapons[2];

	std::string model_name;
	std::unique_ptr<Mat3x4[]> bones;

	int32_t observer_mode;
	EHandle observer_target;

	bool zooming;
	float zoom_base_frac;
	float zoom_base_time;
	float zoom_full_start_time;

	// Local player only
	Vec3 camera_origin;
	Vec3 camera_angles;
};

class BaseNPCEntity : public BaseEntity {
public:
	explicit BaseNPCEntity(uint64_t entity_ptr);
	void update(const GameProcess& process, const GameData& data) override;

	inline Vec3 get_bone_pos(size_t bone) const {
		if (bone < MAXSTUDIOBONES) {
			const auto& mat = bones[bone];
			return origin + Vec3{mat.a[3], mat.a[7], mat.a[11]};
		}
		else {
			return origin;
		}
	}

public:
	Vec3 origin;
	Vec3 angles;
	std::string model_name;
	std::unique_ptr<Mat3x4[]> bones;
};

class WeaponXEntity : public BaseEntity, public ProjectileWeapon {
public:
	explicit WeaponXEntity(uint64_t entity_ptr);
	void update(const GameProcess& process, const GameData& data) override;

	float get_projectile_speed() const;
	float get_projectile_gravity() const;

public:
	EHandle weapon_owner;
	WeaponID weapon_name_index;
	float cur_zoom_fov;
	float target_zoom_fov;
	float projectile_scale;
	float projectile_speed;
};

class PropSurvivalEntity : public BaseEntity {
public:
	explicit PropSurvivalEntity(uint64_t entity_ptr);
	void update(const GameProcess& process, const GameData& data) override;

public:
	Vec3 origin;
	int32_t ammo_in_clip;
	ItemID custom_script_int;
	uint32_t survival_property;
	WeaponID weapon_name_index;
	uint32_t mod_bit_field;
};

class PlayerResourceEntity : public BaseEntity {
public:
	explicit PlayerResourceEntity(uint64_t entity_ptr);
	void update(const GameProcess& process, const GameData& data) override;

	// Gets the name of a player.
	const char* get_name(size_t index) const;
public:
	std::unique_ptr<uint64_t[]> name_pointers;
	std::unique_ptr<std::string[]> names;
};

class WorldEntity : public BaseEntity {
public:
	explicit WorldEntity(uint64_t entity_ptr);
	void update(const GameProcess& process, const GameData& data) override;

	// Returns the radius of the death field.
	float death_field_radius(float curtime) const;
public:
	bool death_field_is_active;
	Vec3 death_field_origin;
	float death_field_radius_start;
	float death_field_radius_end;
	float death_field_time_start;
	float death_field_time_end;
};
