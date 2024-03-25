/*!
Game Data structures SDK.
!*/

#![allow(non_snake_case, dead_code, non_upper_case_globals)]

mod class;
mod cvar;
mod ehandle;
mod entitylist;
mod globalvars;
mod highlight;
mod input;
mod inputsystem;
mod items;
pub mod pitches;
pub mod projectiles;
mod script_net_name;
mod studio;
mod tier1;

pub use dataview::Pod;
pub use intptr::IntPtr64 as Ptr;

pub use crate::base::math::*;

pub use self::class::*;
pub use self::cvar::*;
pub use self::ehandle::*;
pub use self::entitylist::*;
pub use self::globalvars::*;
pub use self::highlight::*;
pub use self::input::*;
pub use self::inputsystem::*;
pub use self::items::*;
pub use self::script_net_name::ScriptNetVarName;
pub use self::studio::*;
pub use self::tier1::*;

pub const MAX_PLAYERS: usize = 128;
pub const MAX_NAME_LENGTH: usize = 64;

pub const SIGNONSTATE_NONE: i32 = 0;
pub const SIGNONSTATE_CHALLENGE: i32 = 1;
pub const SIGNONSTATE_CONNECTED: i32 = 2;
pub const SIGNONSTATE_STATE_NEW: i32 = 3;
pub const SIGNONSTATE_PRESPAWN: i32 = 4;
pub const SIGNONSTATE_GETTING_DATA: i32 = 5;
pub const SIGNONSTATE_SPAWN: i32 = 6;
pub const SIGNONSTATE_FIRST_SNAP: i32 = 7;
pub const SIGNONSTATE_FULL: i32 = 8;
pub const SIGNONSTATE_CHANGELEVEL: i32 = 9;

pub const AIRSHIP_HEIGHT: f32 = 11000.0;

pub const OBS_MODE_NONE: i32 = 0;
pub const OBS_MODE_DEATHCAM: i32 = 1;
pub const OBS_MODE_FIXED: i32 = 3;
pub const OBS_MODE_IN_EYE: i32 = 4;
pub const OBS_MODE_CHASE: i32 = 5;
pub const OBS_MODE_ROAMING: i32 = 7;

pub const HITGROUP_GENERIC: u16 = 0;
pub const HITGROUP_HEAD: u16 = 1;
pub const HITGROUP_UPPER_BODY: u16 = 2;
pub const HITGROUP_LOWER_BODY: u16 = 3;
pub const HITGROUP_LEFT_HAND: u16 = 4;
pub const HITGROUP_RIGHT_HAND: u16 = 5;
pub const HITGROUP_LEFT_LEG: u16 = 6;
pub const HITGROUP_RIGHT_LEG: u16 = 7;

pub const GAMEMODE_SURVIVAL: u32 = crate::hash("survival");
pub const GAMEMODE_ARENAS: u32 = crate::hash("arenas");
pub const GAMEMODE_CONTROL: u32 = crate::hash("control");
pub const GAMEMODE_FREEDM: u32 = crate::hash("freedm");

pub const HARDWARE_PC_1: u32 = 514;
pub const HARDWARE_PC_2: u32 = 1794;
pub const HARDWARE_X1_1: u32 = 0;
pub const HARDWARE_X1_2: u32 = 2570;
pub const HARDWARE_PS4_1: u32 = 257;
pub const HARDWARE_PS4_2: u32 = 2056;
pub const HARDWARE_SWITCH: u32 = 2313;

#[derive(Copy, Clone, Default, Pod)]
#[repr(C)]
pub struct ConsumableItem {
    pub item: u16,
    pub count: u16,
}
