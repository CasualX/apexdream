#pragma once

#include "sdk.hpp"
#include "pid.hpp"

class GameContext;
class GameState;
class PlayerEntity;
class BaseEntity;

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
	// If a target moves more than this distance in a single tick then it is considered teleported and will be dropped.
	float teledist = 72.0f;
	// Enable the PID controller for aim smoothing.
	bool pid_enable = true;
	// The PID controller configuration.
	PidConfig pid_config = { 2.0, 10.0, 0.0, 0.9 };
};

// Target priority by rank.
enum class Rank {
	// Downed players are still targetted but at a lower priority.
	Low,
	// Regular players are always targetted before downed players.
	Normal,
};

// This structures represents intermediate data calculated when validating a target.
struct TargetInfo {
	// Target origin in the world.
	Vec3 origin;
	// Target velocity for linear prediction.
	Vec3 velocity;
	// Target bone position in the world we're trying to hit.
	Vec3 bone_pos;

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
	bool teleported(bool new_target, const TargetInfo& info);

	// Updates the current aim target.
	void find_target(const GameState& state, const PlayerEntity* local);

	// Checks if this target is valid to aim at.
	bool validate(const GameState& state, const PlayerEntity* local, const BaseEntity* target, TargetInfo& info) const;
	bool rules(const GameState& state, const PlayerEntity* local, const BaseEntity* target, TargetInfo& info) const;
	bool hitpoint(const GameState& state, const PlayerEntity* local, const BaseEntity* target, TargetInfo& info) const;
	bool compute(const GameState& state, const PlayerEntity* local, const BaseEntity* target, TargetInfo& info) const;
	bool fov_check(const GameState& state, const PlayerEntity* local, const BaseEntity* target, TargetInfo& info) const;

	// Gets the aim fov.
	float get_fov() const;
	// Gets the scalar for the aim fov, used when zoomed in with a scoped weapon.
	static float get_fov_scale(const GameState& state, const PlayerEntity* local);

	// Moves the mouse towards the target.
	void aim_pid(const TargetInfo& info, float fov_scale, float& dx, float& dy);
	void aim_smooth(const TargetInfo& info, float fov_scale, float& dx, float& dy);
	void aim(const TargetInfo& info, float fov_scale);

private:
	double next_tick = 0.0;

	// Aiming state
	float addx = 0.0f;
	float addy = 0.0f;
	PidController pidx;
	PidController pidy;

	// Keep track of the current target
	bool target_locked = false;
	EHandle target_entity;
	double target_time = 0.0;
	Vec3 target_pos;

public:
	AimAssistConfig config;
};
