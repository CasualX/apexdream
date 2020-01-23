#pragma once

#include "sdk.hpp"

class GameContext;
class GameState;
class PlayerEntity;

struct AimAssistConfig {
	// Configuration
	bool enable = true;
	// Hold down this key to activate aimbot.
	ButtonCode aim_key = ButtonCode::Mouse4;
	// Strength of the aim assist, input values around 1.5 to 2.5.
	float aim_strength = 2.2f;
	// Which bone index to aim at.
	uint32_t aim_bone = 12;
	// When a target becomes invalid, wait this many seconds before trying to find a new target.
	float idle_time = 0.3f;
	// Stop aiming when this close to a target to prevent mouse jitter.
	float fov_min = 0.25f;
	// Angle required to lock onto a target.
	float fov_aim = 10.0f;
	// When locked onto a target, angle required to drop the target.
	float fov_drop = 25.0f;
};

// Target priority by rank.
enum class Rank {
	// Downed players are still targetted but at a lower priority.
	Downed,
	// Regular players are always targetted before downed players.
	Player,
};

// This structures represents intermediate data calculated when validating a target.
struct TargetInfo {
	// Position in the world we're trying to aim at.
	Vec3 hit;
	// Aim angle required to hit the position we're trying to shoot.
	Vec3 aim;
	// Distance to the target.
	float distance;
	// Projectile travel time required to hit the target.
	float time;
	// Delta angle from local player's current viewangles to the target.
	float angle;
	// Delta pitch and yaw in individual components.
	float pitch, yaw;
	// Priority calculated for this target.
	float priority;
	// Rank given for this target.
	Rank rank;
};

class AimAssist {
public:
	AimAssist() = default;
	void run(GameContext& ctx);

	// Updates the tracking state and aims at the aqcuired targets.
	void track(GameContext& ctx, const PlayerEntity* local);

	// Finds a target to aim at, returns nullptr if no valid target was found.
	const PlayerEntity* find_target(GameContext& ctx, const PlayerEntity* local);

	// Checks if this target is valid to aim at.
	bool validate(GameContext& ctx, const PlayerEntity* local, const PlayerEntity* target, TargetInfo& info) const;
	bool rules(GameContext& ctx, const PlayerEntity* local, const PlayerEntity* target, TargetInfo& info) const;
	bool compute(GameContext& ctx, const PlayerEntity* local, const PlayerEntity* target, TargetInfo& info) const;
	bool fov_check(GameContext& ctx, const PlayerEntity* local, const PlayerEntity* target, TargetInfo& info) const;

	// Gets the aim fov.
	float get_fov() const;
	// Gets the scalar for the aim fov, used when zoomed in with a scoped weapon.
	float get_fov_scale(const GameState& state, const PlayerEntity* local) const;

	// Moves the mouse towards the target.
	void aim(const TargetInfo& info, float fov_scale);

private:
	bool target_locked = false;
	EHandle target_entity;
	double target_time = 0.0;
	float addx = 0.0f;
	float addy = 0.0f;
	double next_tick = 0.0;
public:
	AimAssistConfig config;
};
