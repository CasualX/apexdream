#pragma once

#include <cstdint>
#include <cmath>

enum class SignonState: int32_t {
	None,
	Challenge,
	Connected,
	StateNew,
	Prespawn,
	GettingData,
	Spawn,
	FirstSnap,
	Full,
	ChangeLevel,
};

// https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/inputsystem/ButtonCode.h
enum class ButtonCode: uint32_t {
	None = 0,
	// More codes...
	MouseLeft = 108,
	MouseRight = 109,
	MouseMiddle = 110,
	Mouse4 = 111,
	Mouse5 = 112,
};

// https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/globalvars_base.h
struct CGlobalVars {
	double realtime;
	int32_t framecount;
	float absoluteframetime;
	float curtime;
	float frametime;
	int32_t maxClients;
	int32_t tickcount;
	float interval_per_tick;
	float interpolation_amount;
	int32_t simTicksThisFrame;
	int32_t network_protocol;
	uint64_t pSaveData;
	bool m_bClient;
	int32_t nTimestampNetworkingBase;
	int32_t nTimestampRandomizeWindow;
};

// https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/shared/entitylist_base.h#L20-L29
struct CEntInfo {
	uint64_t pEntity;
	int64_t SerialNumber;
	uint64_t pPrev;
	uint64_t pNext;
};

// https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/client_class.h
struct ClientClass {
	uint64_t pCreateFn;
	uint64_t pCreateEventFn;
	uint64_t pNetworkName;
	uint64_t pRecvTable;
	uint64_t pNext;
	uint32_t ClassID;
	uint32_t ClassSize;
};

inline float to_degrees(float rad) {
	return rad * 180.0f / 3.1415927f;
}
inline float to_radians(float deg) {
	return deg * 3.1415927f / 180.0f;
}

struct Vec3 {
	float x, y, z;

	static inline float distance(Vec3 lhs, Vec3 rhs) {
		Vec3 delta = Vec3{rhs.x - lhs.x, rhs.y - lhs.y, rhs.z - lhs.z};
		return sqrtf(delta.x * delta.x + delta.y * delta.y + delta.z * delta.z);
	}
	inline Vec3 operator+ (Vec3 v) const {
		return Vec3{x + v.x, y + v.y, z + v.z};
	}
	inline Vec3 operator- (Vec3 v) const {
		return Vec3{x - v.x, y - v.y, z - v.z};
	}
	inline Vec3 operator* (float scale) const {
		return Vec3{x * scale, y * scale, z * scale};
	}
	inline Vec3 to_angles() const {
		float tmp, yaw, pitch;
		if (y == 0.0f && x == 0.0f) {
			yaw = 0.0f;
			pitch = z > 0.0f ? 270.0f : 90.0f;
		}
		else {
			yaw = to_degrees(atan2f(y, x));
			tmp = sqrtf(x * x + y * y);
			pitch = to_degrees(atan2f(-z, tmp));
		}
		return Vec3{pitch, yaw, 0.0f};
	}
	inline Vec3 norm_angles() const {
		float pitch = x < -90.0f ? -90.0f : (x > 90.0f ? 90.0f : x);
		float yaw = y;
		while (yaw <= -180.0f) yaw += 360.0f;
		while (yaw > 180.0f) yaw -= 360.0f;
		return Vec3{pitch, yaw, 0.0f};
	}
	static inline float diff_angles(Vec3 lhs, Vec3 rhs) {
		const auto delta = (lhs - rhs).norm_angles();
		return sqrtf(delta.x * delta.x + delta.y * delta.y);
	}
};
struct Mat3x4 {
	float a[12];
};

union FloatInt {
	float f32;
	int32_t i32;
	uint32_t u32;
	uint8_t bytes[4];
};

const size_t NUM_ENT_ENTRIES = 0x10000;
const size_t MAXSTUDIOBONES = 128;

struct EHandle {
	uint32_t value = 0xffffffff;

	inline bool is_valid() const { return value != 0xffffffff; }
	inline size_t index() const { return value & static_cast<uint32_t>(NUM_ENT_ENTRIES - 1); }
};
