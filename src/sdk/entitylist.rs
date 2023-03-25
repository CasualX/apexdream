use super::{Pod, Ptr};

pub const NUM_ENT_ENTRIES: usize = 0x10000;
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CEntInfo {
	pub pEntity: Ptr, // IHandleEntity*
	pub SerialNumber: i64,
	pub pPrev: Ptr<CEntInfo>,
	pub pNext: Ptr<CEntInfo>,
}
unsafe impl Pod for CEntInfo {}

pub const CPlayer: u32 = hash!("CPlayer");
pub const CPropSurvival: u32 = hash!("CPropSurvival");
pub const CWeaponX: u32 = hash!("CWeaponX");
pub const CWorld: u32 = hash!("CWorld");
pub const CAI_BaseNPC: u32 = hash!("CAI_BaseNPC");
pub const CPlayerWaypoint: u32 = hash!("CPlayerWaypoint");
pub const CPlayerVehicle: u32 = hash!("CPlayerVehicle");
pub const CDeathBoxProp: u32 = hash!("CDeathBoxProp");
pub const CDynamicProp: u32 = hash!("CDynamicProp");
pub const CScriptProp: u32 = hash!("CScriptProp");
pub const CPhysicsProp: u32 = hash!("CPhysicsProp");
pub const CCrossbowBolt: u32 = hash!("CCrossbowBolt");
pub const CBaseGrenade: u32 = hash!("CBaseGrenade");

pub const CScriptNetData_SNDC_PLAYER_EXCLUSIVE: u32 = hash!("CScriptNetData_SNDC_PLAYER_EXCLUSIVE");
pub const CScriptNetData_SNDC_PLAYER_GLOBAL: u32 = hash!("CScriptNetData_SNDC_PLAYER_GLOBAL");
