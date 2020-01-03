#include "solver.hpp"

#include <cmath>

Vec3 ProjectileWeapon::get_projectile_fire_setup(const Vec3& origin, const Vec3& target) const {
	return target - origin;
}

Vec3 LinearPredictor::predict_position(float time) const {
	return origin + velocity * time;
}

// https://en.wikipedia.org/wiki/Projectile_motion#Angle_%7F%27%22%60UNIQ--postMath-0000003A-QINU%60%22%27%7F_required_to_hit_coordinate_(x,y)
static bool optimal(float x, float y, float v0, float g, float& pitch) {
	const float root = v0 * v0 * v0 * v0 - g * (g * x * x + 2.0f * y * v0 * v0);
	if (root < 0.0f) {
		return false;
	}
	pitch = atan((v0 * v0 - sqrt(root)) / (g * x));
	return true;
}

static bool solve2d(const Vec3& origin, const ProjectileWeapon& weapon, const Vec3& target, Solution& sol) {
	const auto v = weapon.get_projectile_fire_setup(origin, target);
	const float dx = sqrt(v.x * v.x + v.y * v.y);
	const float dy = v.z;
	const float v0 = weapon.get_projectile_speed();
	const float g = weapon.get_projectile_gravity();
	if (!optimal(dx, dy, v0, g, sol.pitch)) {
		return false;
	}
	sol.time = dx / (cos(sol.pitch) * v0);
	sol.yaw = atan2(v.y, v.x);
	return true;
}

bool solve(const Vec3& origin, const ProjectileWeapon& weapon, const TargetPredictor& target, Solution& sol) {
	static const float MAX_TIME = 1.0f;
	static const float TIME_STEP = 1.0 / 256.0f;
	for (float target_time = 0.0f; target_time <= MAX_TIME; target_time += TIME_STEP) {
		const auto target_pos = target.predict_position(target_time);
		if (!solve2d(origin, weapon, target_pos, sol)) {
			return false;
		}
		if (sol.time < target_time) {
			return true;
		}
	}
	return false;
}
