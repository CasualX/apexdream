#include "aimassist.hpp"
#include "context.hpp"
#include "entities.hpp"
#include "state.hpp"
#include "process.hpp"
#include "solver.hpp"
#include "sdk.hpp"

#include <cmath>

static const int TICKRATE = 144;

void AimAssist::run(GameContext& ctx) {
	// Rate limit the aimbot to 60 times per sec
	if (ctx.time < next_tick) {
		return;
	}
	// Fixup timer to maintain 60 times per sec
	const double step_time = 1.0 / TICKRATE;
	next_tick = ctx.time < next_tick + step_time ? next_tick + step_time : ctx.time + step_time;

	const auto local = ctx.state.local_player();
	if (config.enable && ctx.state.is_button_down(config.aim_key) && local) {
		track(ctx, local);
	}
	else {
		target_entity = EHandle{};
		target_locked = false;
	}
}

void AimAssist::track(GameContext& ctx, const PlayerEntity* local) {
	// If we were locked onto a target, only find a new target when sufficient time has passed
	// Else find a new target if we don't already have one
	const bool new_target = target_locked ? ctx.time > target_time + config.idle_time : !target_entity.is_valid();

	// Find a new target if desired
	if (new_target) {
		if (const auto target = find_target(ctx.state, local)) {
			target_entity = target->handle;
			target_locked = true;
			pidx.reset();
			pidy.reset();
		}
	}

	// Aim at the target if we have one
	if (const auto target = ctx.state.get_entity<BaseEntity>(target_entity)) {
		TargetInfo info{};
		if (validate(ctx.state, local, target, info) && !teleported(new_target, info)) {
			// Target is valid, aim at the target
			const float fov_scale = get_fov_scale(ctx.state, local);
			aim(info, fov_scale);
			// Update the time we've last seen this target
			target_time = ctx.time;
		}
		else {
			// Target is no longer valid, drop it but keep the locked flag
			target_entity = EHandle{};
		}
	}
}
bool AimAssist::teleported(bool new_target, const TargetInfo& info) {
	// If we only just acquired the target it did not teleport
	const bool teleported = !new_target && Vec3::distance(target_pos, info.hit) > config.teledist;
	// Keep track of the target's position
	// Use bone position to ensure smooth transitions
	target_pos = info.hit;
	return teleported;
}

const BaseEntity* AimAssist::find_target(const GameState& state, const PlayerEntity* local) {
	const BaseEntity* target = nullptr;
	Rank rank = Rank::Low;
	float priority = 99999999.0f;
	// Consider every player a target
	for (uint32_t i = 1; i < NUM_ENT_ENTRIES; i += 1) {
		if (const auto candidate = state.get_entity<BaseEntity>(EHandle{i})) {
			// If this candidate target is a valid target
			TargetInfo info{};
			if (validate(state, local, candidate, info)) {
				// With a higher priority
				if (info.rank > rank || info.rank == rank && info.priority < priority) {
					// Consider this player the best target
					rank = info.rank;
					priority = info.priority;
					target = candidate;
				}
			}
		}
	}
	return target;
}

bool AimAssist::validate(const GameState& state, const PlayerEntity* local, const BaseEntity* target, TargetInfo& info) const {
	if (!rules(state, local, target, info)) {
		return false;
	}
	if (!hitpoint(state, local, target, info)) {
		return false;
	}
	if (!compute(state, local, target, info)) {
		return false;
	}
	if (!fov_check(state, local, target, info)) {
		return false;
	}
	info.priority = info.angle + logf(info.distance);
	return true;
}
bool AimAssist::rules(const GameState& state, const PlayerEntity* local, const BaseEntity* target, TargetInfo& info) const {
	if (const auto player = dynamic_cast<const PlayerEntity*>(target)) {
		if (local == player) {
			return false;
		}
		if (!player->is_visible(state.curtime)) {
			return false;
		}
		if (local->team_num == player->team_num) {
			return false;
		}
		if (!player->is_alive()) {
			return false;
		}
		info.rank = player->is_downed() ? Rank::Low : Rank::Normal;
		return true;
	}
	else if (const auto base_npc = dynamic_cast<const BaseNPCEntity*>(target)) {
		if (!base_npc->is_visible(state.curtime)) {
			return false;
		}
		info.rank = Rank::Low;
		return true;
	}
	else {
		return false;
	}
}
bool AimAssist::hitpoint(const GameState& state, const PlayerEntity* local, const BaseEntity* target, TargetInfo& info) const {
	if (const auto player = dynamic_cast<const PlayerEntity*>(target)) {
		info.origin = player->origin;
		info.velocity = player->velocity;
		info.bone_pos = player->get_bone_pos(config.aim_bone);
		return true;
	}
	else if (const auto base_npc = dynamic_cast<const BaseNPCEntity*>(target)) {
		info.origin = base_npc->origin;
		info.velocity = {};
		info.bone_pos = base_npc->get_bone_pos(config.aim_bone);
		return true;
	}
	else {
		info.origin = {};
		info.velocity = {};
		info.bone_pos = {};
		return false;
	}
}
bool AimAssist::compute(const GameState& state, const PlayerEntity* local, const BaseEntity* target, TargetInfo& info) const {
	info.distance = Vec3::distance(info.origin, local->camera_origin);

	// Projectile aimbot calculations :)
	if (const auto weapon = state.get_entity<WeaponXEntity>(local->active_weapon())) {
		if (weapon->projectile_speed > 1.0f) {
			LinearPredictor predictor{info.bone_pos, info.velocity};
			Solution sol;
			if (solve(local->camera_origin, *weapon, predictor, sol)) {
				info.time = sol.time;
				info.aim = Vec3 {-rad2deg(sol.pitch), rad2deg(sol.yaw), 0.0f};
				return true;
			}
			else {
				return false;
			}
		}
	}

	// Hitscan weapons and others fall back to aiming at the target.
	info.time = 0.0f;
	info.aim = (info.hit - local->camera_origin).to_angles().norm_angles();
	return true;
}
bool AimAssist::fov_check(const GameState& state, const PlayerEntity* local, const BaseEntity* target, TargetInfo& info) const {
	info.pitch = info.aim.x - local->camera_angles.x;
	info.yaw = info.aim.y - local->camera_angles.y;
	while (info.yaw <= -180.0f) info.yaw += 360.0f;
	while (info.yaw > 180.0f) info.yaw -= 360.0f;
	info.angle = sqrt(info.pitch * info.pitch + info.yaw * info.yaw);
	const float fov = get_fov() * get_fov_scale(state, local);
	return info.angle < fov;
}

float AimAssist::get_fov() const {
	return target_entity.is_valid() && target_locked ? config.fov_drop : config.fov_aim;
}
float AimAssist::get_fov_scale(const GameState& state, const PlayerEntity* local) {
	if (local->zooming) {
		if (const auto weapon = state.get_entity<WeaponXEntity>(local->active_weapon())) {
			if (weapon->target_zoom_fov != 0.0f && weapon->target_zoom_fov != 1.0f) {
				return weapon->target_zoom_fov / 90.0f;
			}
		}
	}
	return 1.0f;
}

void AimAssist::aim_pid(const TargetInfo& info, float fov_scale, float& dx, float& dy) {
	const float dt = 1.0f / TICKRATE;
	const float strength = 1.0f / fov_scale;
	dx = -pidx.step(info.yaw * strength, dt, config.pid_config);
	dy = pidy.step(info.pitch * strength, dt, config.pid_config);
}
void AimAssist::aim_smooth(const TargetInfo& info, float fov_scale, float& dx, float& dy) {
	// Avoid aim jitter with a minimum angle
	if (info.angle < config.fov_min * fov_scale) {
		dx = 0.0f;
		dy = 0.0f;
	}
	// Magic aim smoothing formula :)
	const float aim_strength = config.aim_strength;
	const float speed = logf(aim_strength + info.angle / (fov_scale * fov_scale) * aim_strength) * aim_strength + aim_strength;
	dx = -info.yaw * speed;
	dy = info.pitch * speed;
}
void AimAssist::aim(const TargetInfo& info, float fov_scale) {
	float dx, dy;
	if (config.pid_enable) {
		aim_pid(info, fov_scale, dx, dy);
	}
	else {
		aim_smooth(info, fov_scale, dx, dy);
	}
	// Moving the mouse can only be done in whole steps
	// Keep track of the delta 'residue' and add it next time
	dx += addx;
	dy += addy;
	const int mdx = static_cast<int>(dx);
	const int mdy = static_cast<int>(dy);
	addx = dx - static_cast<float>(mdx);
	addy = dy - static_cast<float>(mdy);
	if (mdx != 0 || mdy != 0) {
		mouse_move(mdx, mdy);
	}
}
