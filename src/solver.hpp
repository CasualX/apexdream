#pragma once

#include "sdk.hpp"

class ProjectileWeapon {
public:
	virtual float get_projectile_speed() const = 0;
	virtual float get_projectile_gravity() const = 0;
	virtual Vec3 get_projectile_fire_setup(const Vec3& origin, const Vec3& target) const;
};

class TargetPredictor {
public:
	virtual Vec3 predict_position(float time) const = 0;
};

// Simple linear extrapolation.
struct LinearPredictor : public TargetPredictor {
	inline LinearPredictor(Vec3 origin, Vec3 velocity) : origin(origin), velocity(velocity) {}

	Vec3 origin;
	Vec3 velocity;

	Vec3 predict_position(float time) const;
};

struct Solution {
	float pitch, yaw, time;
};

// Given a projectile weapon and a predictable target, solve where to aim the weapon to thit the target.
// NOTE! Solution is returned in radians! Don't forget to convert to degrees if needed!
bool solve(const Vec3& origin, const ProjectileWeapon& weapon, const TargetPredictor& target, Solution& sol);
