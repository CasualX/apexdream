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
	/*0x00*/double realtime;
	/*0x08*/int32_t framecount;
	/*0x0c*/float absoluteframetime;
	/*0x10*/float curtime;
	/*0x14*/float curtime2;
	/*0x18*/float curtime3;
	/*0x1c*/float curtime4;
	/*0x20*/float frametime;
	/*0x24*/float curtime5;
	/*0x28*/float curtime6;
	/*0x2c*/float zero;
	/*0x30*/float frametime2;
	/*0x34*/int32_t maxClients;
	/*0x38*/int32_t unk38;
	/*0x3c*/int32_t unk3C;
	/*0x40*/int32_t tickcount;
	/*0x44*/float interval_per_tick;
	/*0x48*/float interpolation_amount;
	// There's more stuff after this but I don't know and I don't care
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

// https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/client/kbutton.h
struct kbutton_t {
	int down[2];
	int state;
};

inline float rad2deg(float rad) {
	return rad * 180.0f / 3.1415927f;
}
inline float deg2rad(float deg) {
	return deg * 3.1415927f / 180.0f;
}

struct Vec3 {
	float x, y, z;

	inline static float distance(Vec3 lhs, Vec3 rhs) {
		Vec3 delta = Vec3{rhs.x - lhs.x, rhs.y - lhs.y, rhs.z - lhs.z};
		return sqrt(delta.x * delta.x + delta.y * delta.y + delta.z * delta.z);
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
		float yaw, pitch;
		if (y == 0.0f && x == 0.0f) {
			yaw = 0.0f;
			pitch = z > 0.0f ? 270.0f : 90.0f;
		}
		else {
			yaw = rad2deg(atan2(y, x));
			const float tmp = sqrt(x * x + y * y);
			pitch = rad2deg(atan2(-z, tmp));
		}
		return Vec3{pitch, yaw, 0.0f};
	}
	inline Vec3 norm_angles() const {
		const float pitch = x < -90.0f ? -90.0f : (x > 90.0f ? 90.0f : x);
		float yaw = y;
		while (yaw <= -180.0f) yaw += 360.0f;
		while (yaw > 180.0f) yaw -= 360.0f;
		return Vec3{pitch, yaw, 0.0f};
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
const size_t MAX_PLAYERS = 128;

struct EHandle {
	uint32_t value = 0xffffffff;

	inline bool is_valid() const {
		return value != 0xffffffff;
	}
	inline int index() const {
		return is_valid() ? static_cast<int>(value & static_cast<uint32_t>(NUM_ENT_ENTRIES - 1)) : -1;
	}

	inline bool operator== (EHandle rhs) const {
		return index() == rhs.index();
	}
	inline bool operator!= (EHandle rhs) const {
		return index() != rhs.index();
	}
};

struct NameEntry {
	uint64_t name1;
	uint64_t name2; // For whatever reason there's two name entries...
};

struct CNetStringTableItem {
	/*0x00*/ uint64_t unk00;
	/*0x08*/ uint64_t unk08;
	/*0x10*/ uint64_t string;
	/*0x18*/ uint64_t unk18;
	/*0x20*/ uint64_t unk20;
	/*0x28*/ uint64_t unk28;
	/*0x30*/ uint64_t unk30;
	/*0x38*/ uint64_t unk38;
	/*0x40*/ uint64_t unk40;
};
struct CNetStringDict {
	/*0x00*/ uint64_t vtable;
	/*0x08*/ uint64_t _unk08;
	/*0x10*/ uint64_t _unk10;
	/*0x18*/ uint64_t elements;
	/*0x20*/ uint16_t allocation_count;
	/*0x22*/ uint16_t grow_size;
	/*0x24*/ uint32_t _unk24;
	/*0x28*/ uint64_t _unk28;
	/*0x30*/ uint16_t _unk30;
	/*0x32*/ uint16_t used;
	/*0x34*/ uint16_t _unk34;
	/*0x36*/ uint16_t highest;
};
struct CNetStringTable {
	/*0x00*/ uint64_t vtable;
	/*0x08*/ int32_t table_id;
	/*0x0c*/ uint32_t table_id_pad;
	/*0x10*/ uint64_t table_name;
	/*0x18*/ int32_t max_entries;
	/*0x1c*/ int32_t entry_bits;
	/*0x20*/ int32_t tick_count;
	/*0x24*/ int32_t last_changed_tick;
	/*0x28*/ uint32_t flags;
	/*0x2c*/ int32_t user_data_size;
	/*0x30*/ int32_t user_data_size_bits;
	/*0x34*/ uint32_t pad;
	/*0x38*/ uint64_t change_func;
	/*0x40*/ uint64_t object;
	/*0x48*/ uint64_t items;
	/*0x50*/ uint64_t items_client_side;
};
