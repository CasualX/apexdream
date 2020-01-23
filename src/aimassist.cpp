#include "aimassist.hpp"
#include "context.hpp"
#include "entities.hpp"
#include "state.hpp"
#include "process.hpp"
#include "solver.hpp"

#include <cmath>

void AimAssist::run(GameContext& ctx) {
	// Rate limit the aimbot to 60 times per sec
	if (ctx.time < next_tick) {
		return;
	}
	// Fixup timer to maintain 60 times per sec
	static const double TICKRATE = 1.0 / 60.0;
	next_tick = ctx.time < next_tick + TICKRATE ? next_tick + TICKRATE : ctx.time + TICKRATE;

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
		if (const auto target = find_target(ctx, local)) {
			target_entity = target->handle;
			target_locked = true;
		}
	}

	// Aim at the target if we have one
	if (const auto target = ctx.state.get_entity<PlayerEntity>(target_entity)) {
		TargetInfo info{};
		if (validate(ctx, local, target, info)) {
			// Adjust the fov setting based on scoping state
			const float fov_scale = get_fov_scale(ctx.state, local);
			// Avoid aim jitter with minimum angle
			if (info.angle >= config.fov_min * fov_scale) {
				aim(info, fov_scale);
			}
			// Update the time we've last seen this target
			target_time = ctx.time;
		}
		else {
			// Target is no longer valid, drop it but keep the locked flag
			target_entity = EHandle{};
		}
	}
}

const PlayerEntity* AimAssist::find_target(GameContext& ctx, const PlayerEntity* local) {
	const PlayerEntity* target = nullptr;
	Rank rank = Rank::Downed;
	float priority = 99999999.0f;
	// Consider every player a target
	for (uint32_t i = 1; i <= 64; i += 1) {
		if (const auto player = ctx.state.get_entity<PlayerEntity>(EHandle{i})) {
			// If this player is a valid target
			TargetInfo info{};
			if (validate(ctx, local, player, info)) {
				// With a higher priority
				if (info.rank > rank || info.rank == rank && info.priority < priority) {
					// Consider this player the best target
					rank = info.rank;
					priority = info.priority;
					target = player;
				}
			}
		}
	}
	return target;
}

bool AimAssist::validate(GameContext& ctx, const PlayerEntity* local, const PlayerEntity* target, TargetInfo& info) const {
	if (!rules(ctx, local, target, info)) {
		return false;
	}
	if (!compute(ctx, local, target, info)) {
		return false;
	}
	if (!fov_check(ctx, local, target, info)) {
		return false;
	}
	info.priority = info.angle + log(info.distance);
	return true;
}
bool AimAssist::rules(GameContext& ctx, const PlayerEntity* local, const PlayerEntity* target, TargetInfo& info) const {
	if (local == target) {
		return false;
	}
	if (local->team_num == target->team_num) {
		return false;
	}
	if (!target->is_alive()) {
		return false;
	}
	info.rank = target->is_downed() ? Rank::Downed : Rank::Player;
	return true;
}
bool AimAssist::compute(GameContext& ctx, const PlayerEntity* local, const PlayerEntity* target, TargetInfo& info) const {
	if (target == local) {
		// info.hit = Vec3{35175.1406,-6868.41504,-28173.9688};
		info.hit = Vec3{};
	}
	else {
		info.hit = target->get_bone_pos(config.aim_bone);
	}
	info.distance = Vec3::distance(info.hit, local->camera_origin);

	// Projectile aimbot calculations :)
	if (const auto weapon = ctx.state.get_entity<WeaponXEntity>(local->active_weapon())) {
		if (weapon->projectile_speed > 1.0f) {
			LinearPredictor predictor{info.hit, target->velocity};
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
bool AimAssist::fov_check(GameContext& ctx, const PlayerEntity* local, const PlayerEntity* target, TargetInfo& info) const {
	info.pitch = info.aim.x - local->camera_angles.x;
	info.yaw = info.aim.y - local->camera_angles.y;
	while (info.yaw <= -180.0f) info.yaw += 360.0f;
	while (info.yaw > 180.0f) info.yaw -= 360.0f;
	info.angle = sqrt(info.pitch * info.pitch + info.yaw * info.yaw);
	const float fov = get_fov() * get_fov_scale(ctx.state, local);
	return info.angle < fov;
}

float AimAssist::get_fov() const {
	return target_entity.is_valid() && target_locked ? config.fov_drop : config.fov_aim;
}
float AimAssist::get_fov_scale(const GameState& state, const PlayerEntity* local) const {
	if (local->zooming) {
		if (const auto weapon = state.get_entity<WeaponXEntity>(local->active_weapon())) {
			if (weapon->target_zoom_fov != 0.0f && weapon->target_zoom_fov != 1.0f) {
				return weapon->target_zoom_fov / 90.0f;
			}
		}
	}
	return 1.0f;
}
void AimAssist::aim(const TargetInfo& info, float fov_scale) {
	// Magic aim smoothing formula :)
	const float aim_strength = config.aim_strength;
	const float speed = log(aim_strength + info.angle / (fov_scale * fov_scale) * aim_strength) * aim_strength + aim_strength;
	// Moving the mouse can only be done in whole steps
	// Keep track of the delta 'residue' and add it next time
	const float dx = -info.yaw * (speed + addx);
	const float dy = info.pitch * (speed + addy);
	const int mdx = static_cast<int>(dx);
	const int mdy = static_cast<int>(dy);
	addx = dx - static_cast<float>(mdx);
	addy = dy - static_cast<float>(mdy);
	if (mdx != 0 || mdy != 0) {
		mouse_move(mdx, mdy);
	}
}
