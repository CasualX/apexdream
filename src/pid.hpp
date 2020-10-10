#pragma once

struct PidConfig {
	// Proportional coefficient.
	float kp;
	// Integral coefficient.
	float ki;
	// Derivative coefficient.
	float kd;
};

struct PidController {
	float p, i;

	float step(float err, float dt, const PidConfig& config);
	void reset();
};

inline float PidController::step(float err, float dt, const PidConfig& config) {
	const float d = (err - p) / dt;
	p = err;
	i += err * dt;
	return config.kp * p + config.ki * i + config.kd * d;
}
inline void PidController::reset() {
	p = 0.0f;
	i = 0.0f;
}
