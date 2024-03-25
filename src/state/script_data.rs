#![allow(dead_code)]
use super::*;

// Discussion on UnknownCheats:
// https://www.unknowncheats.me/forum/apex-legends/319804-apex-legends-reversal-structs-offsets-447.html#post3397260

#[derive(Debug, Default, sdk::Pod)]
#[repr(C)]
pub struct NetVarEntry {
    // Index = name_hash % 300 (+1 on collision)
    pub name_hash: u64,
    // GetNetworkedVariableCategory returns this value
    // Used to index table at 0xBD82A00 (0x380 bytes per element)
    pub category: u32,
    // 0: bool, 1: u8, 2: i16, 3: i32, 4: i64?, 5: float, 6: time, 7: entity
    pub value_type: u32,
    pub value_index: u32,
    pub unk0x14: u32,
    pub unk0x18: u32,
    pub unk0x1c: f32,
    pub unk0x20: u32,
    pub unk0x24: u32,
    pub unk0x28: sdk::Ptr,
    pub unk0x30: u32,
    pub unk0x34: u32,
}
const _: [(); 0x38] = [(); std::mem::size_of::<NetVarEntry>()];

#[derive(sdk::Pod)]
#[repr(C)]
pub struct NetVarData {
    pub ptr: sdk::Ptr, // Pointer to CScriptNetData instance
    pub salt: u32, // Check upper 16 bits of script net data reference equals this value, -1 = invalid reference
    pub pad1: u32,
    pub pad2: u64,
    pub pad3: u64,
}
const _: [(); 1 << 5] = [(); std::mem::size_of::<NetVarData>()];

#[derive(Default)]
pub struct ScriptNetData {
    pub entries: Vec<NetVarEntry>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ScriptValue {
    Invalid,
    Bool(bool),
    Byte(u8),
    Word(i16),
    Int(i32),
    Float(f32),
    Time(f32),
    Entity(sdk::EHandle),
}
#[allow(dead_code)]
impl ScriptValue {
    #[inline]
    pub const fn to_bool(self) -> Option<bool> {
        match self {
            ScriptValue::Bool(value) => Some(value),
            _ => None,
        }
    }
    #[inline]
    pub const fn to_byte(self) -> Option<u8> {
        match self {
            ScriptValue::Byte(value) => Some(value),
            _ => None,
        }
    }
    #[inline]
    pub const fn to_word(self) -> Option<i16> {
        match self {
            ScriptValue::Word(value) => Some(value),
            _ => None,
        }
    }
    #[inline]
    pub const fn to_int(self) -> Option<i32> {
        match self {
            ScriptValue::Int(value) => Some(value),
            _ => None,
        }
    }
    #[inline]
    pub const fn to_float(self) -> Option<f32> {
        match self {
            ScriptValue::Float(value) => Some(value),
            _ => None,
        }
    }
    #[inline]
    pub const fn to_time(self) -> Option<f32> {
        match self {
            ScriptValue::Time(value) => Some(value),
            _ => None,
        }
    }
}
impl Default for ScriptValue {
    #[inline]
    fn default() -> Self {
        ScriptValue::Invalid
    }
}

impl ScriptNetData {
    pub fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
        if ctx.data.network_var_table_ptr == 0 || ctx.data.network_var_table_len == 0 {
            return;
        }
        let entries_len = ctx.data.network_var_table_len as usize;
        if self.entries.len() != entries_len {
            self.entries.resize_with(entries_len, Default::default);
        }
        // FIXME! Do I need to check this more often than connected?
        if ctx.connected {
            let table_ptr = ctx
                .process
                .base
                .field::<[NetVarEntry]>(ctx.data.network_var_table_ptr);
            let _ = api.vm_read_into(table_ptr, &mut self.entries[..]);
        }
    }
}

impl GameState {
    pub fn read_script_value(
        &self,
        name: sdk::ScriptNetVarName,
        index: sdk::EHandle,
    ) -> ScriptValue {
        let entries = &self.script_data.entries[..];
        if !index.is_valid() || entries.len() == 0 {
            return ScriptValue::Invalid;
        }

        // Look up script var entry through hash table
        let mut name_index = (name.0 % entries.len() as u64) as usize;
        let entry = loop {
            let Some(entry) = entries.get(name_index) else {
                return ScriptValue::default();
            };
            if entry.name_hash == name.0 {
                break entry;
            }
            name_index += 1;
        };

        let script_net_data = match self.entity_as::<ScriptNetDataEntity>(index) {
            Some(e) => e,
            None => return ScriptValue::Invalid,
        };

        // Read script value
        match entry.value_type {
            0 => script_net_data
                .bools
                .get(entry.value_index as usize)
                .map(|&value| ScriptValue::Bool(value != 0))
                .unwrap_or(ScriptValue::Invalid),
            1 => script_net_data
                .bools
                .get(entry.value_index as usize)
                .map(|&value| ScriptValue::Byte(value))
                .unwrap_or(ScriptValue::Invalid),
            2 => script_net_data
                .ranges
                .get(entry.value_index as usize)
                .map(|&value| ScriptValue::Word(value))
                .unwrap_or(ScriptValue::Invalid),
            3 => script_net_data
                .ints
                .get(entry.value_index as usize)
                .map(|&value| ScriptValue::Int(value))
                .unwrap_or(ScriptValue::Invalid),
            5 => script_net_data
                .times
                .get(entry.value_index as usize)
                .map(|&value| ScriptValue::Float(value))
                .unwrap_or(ScriptValue::Invalid),
            6 => script_net_data
                .times
                .get(entry.value_index as usize)
                .map(|&value| ScriptValue::Time(value))
                .unwrap_or(ScriptValue::Invalid),
            7 => script_net_data
                .ents
                .get(entry.value_index as usize)
                .map(|&value| ScriptValue::Entity(value))
                .unwrap_or(ScriptValue::Invalid),
            _ => ScriptValue::Invalid,
        }
    }
    // pub fn write_script_value(&self, api: &mut Api, ctx: &UpdateContext, name: u64, index: u32, value: ScriptValue) {
    // 	// Look up script var entry through hash table
    // 	let entries = &self.script_data.entries[..];
    // 	let mut name_index = name % entries.len() as u64;
    // 	let entry = loop {
    // 		let entry = some!(entries.get(name_index as usize));
    // 		if entry.name_hash == name {
    // 			break entry;
    // 		}
    // 		name_index += 1;
    // 	};
    // 	// Get script data
    // 	let data_ptr = ctx.process.base.field::<[NetVarData]>(0x1974AD8).at((index & 0xffff) as usize);
    // 	let data = ok!(api.vm_read(data_ptr));
    // 	// Reference check
    // 	if data.ptr.is_null() || data.salt != index >> 16 {
    // 		return ScriptValue::Invalid;
    // 	}
    // 	match entry.value_type {
    // 		0 => {
    // 			let value_ptr = data.ptr.field::<[u8]>(0xC70).at(entry.value_index as usize);
    // 			let value = ok!(api.vm_read(value_ptr));
    // 			ScriptValue::Bool(value != 0)
    // 		},
    // 		1 => {
    // 			let value_ptr = data.ptr.field::<[u8]>(0xC70).at(entry.value_index as usize);
    // 			let value = ok!(api.vm_read(value_ptr));
    // 			ScriptValue::Byte(value)
    // 		},
    // 		2 => {
    // 			let value_ptr = data.ptr.field::<[i16]>(0xC82).at(entry.value_index as usize);
    // 			let value = ok!(api.vm_read(value_ptr));
    // 			ScriptValue::Word(value)
    // 		},
    // 		3 => {
    // 			let value_ptr = data.ptr.field::<[i32]>(0xCC8).at(entry.value_index as usize);
    // 			let value = ok!(api.vm_read(value_ptr));
    // 			ScriptValue::Int(value)
    // 		},
    // 		5 => {
    // 			let value_ptr = data.ptr.field::<[f32]>(0xCF0).at(entry.value_index as usize);
    // 			let value = ok!(api.vm_read(value_ptr));
    // 			ScriptValue::Float(value)
    // 		},
    // 		6 => {
    // 			let value_ptr = data.ptr.field::<[f32]>(0xCF0).at(entry.value_index as usize);
    // 			let value = ok!(api.vm_read(value_ptr));
    // 			ScriptValue::Time(value)
    // 		},
    // 		_ => ScriptValue::Invalid,
    // 	}
    // }
}
