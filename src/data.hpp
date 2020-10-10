#pragma once

#include <cstdint>

namespace data {
	const uint32_t TIME_DATE_STAMP = 0x5f6d432f;
	const uint32_t CHECKSUM = 0x1fc1a53;

	const uint32_t GLOBAL_VARS = 0x1261c80;

	const uint32_t ENTITY_LIST = 0x18ad3a8;
	const uint32_t LOCAL_ENTITY = 0x1191ebc;

	const uint32_t CLIENT_STATE = 0x1261f80;
	const uint32_t CLIENT_SIGNON_STATE = 0x0098;
	const uint32_t CLIENT_LEVEL_NAME = 0x01b0;

	const uint32_t NST_WEAPON_NAMES = 0x040d54d8;

	const uint32_t VIEW_RENDER = 0x40d5d98;
	const uint32_t VIEW_MATRIX = 0x1b3bd0;

	const uint32_t INPUT_SYSTEM = 0x012d6900;
	const uint32_t INPUT_BUTTON_STATE = 0xb0;

	const uint32_t NAME_LIST = 0x8167BB0;

	const uint32_t IN_ATTACK = 0x040d5e68;
	const uint32_t IN_JUMP = 0x040d5ee0;
	const uint32_t IN_RELOAD = 0x040d5e88;

	const uint32_t ENTITY_MODEL_NAME = 0x0030;
	const uint32_t ENTITY_FLAGS = 0x0098;
	const uint32_t ENTITY_ORIGIN = 0x014c;
	const uint32_t ENTITY_SHIELDS = 0x0170;
	const uint32_t ENTITY_HEALTH = 0x0420;
	const uint32_t ENTITY_TEAM_NUM = 0x0430;
	const uint32_t ENTITY_VELOCITY = 0x0460;
	const uint32_t ENTITY_MAX_HEALTH = 0x0550;
	const uint32_t ENTITY_SIGNIFIER_NAME = 0x0558;
	const uint32_t ENTITY_LIFE_STATE = 0x0770;

	const uint32_t ANIMATING_BONE_ARRAY = 0xF18;
	const uint32_t ANIMATING_STUDIOHDR = 0x1110;

	const uint32_t BCC_LATEST_PRIMARY_WEAPONS = 0x1a0c;
	const uint32_t BCC_LAST_VISIBLE_TIME = 0x1a6c;

	const uint32_t PLAYER_ZOOM_STATE = 0x1b80;
	const uint32_t PLAYER_CAMERA_DATA = 0x1e6c;
	const uint32_t PLAYER_BLEEDOUT_STATE = 0x2610;
	const uint32_t PLAYER_OBSERVER_MODE = 0x32bc;
	const uint32_t PLAYER_HELMET_ARMOR_TYPE = 0x4274;

	const uint32_t WEAPONX_WEAPON_OWNER = 0x1600;
	const uint32_t WEAPONX_NEXT_PRIMARY_ATTACK = 0x160c;
	const uint32_t WEAPONX_AMMO_IN_CLIP = 0x1634;
	const uint32_t WEAPONX_PLAYER_DATA_ZOOM_FOV = 0x1668 + 0x00b4;
	const uint32_t WEAPONX_MOD_BITFIELD = 0x1794;
	const uint32_t WEAPONX_WEAPON_NAME_INDEX = 0x17b0;
	const uint32_t WEAPONX_PROJECTILE_SPEED = 0x1e0c;

	const uint32_t VEHICLE_DRIVER = 0x1984;
	const uint32_t VEHICLE_VELOCITY = 0x19c0;

	const uint32_t PROP_SURVIVAL = 0x1604;
	const uint32_t WORLD_DEATH_FIELD = 0x0a80;
};
