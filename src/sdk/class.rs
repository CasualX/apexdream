use super::{Pod, Ptr};
use std::mem;

#[derive(Copy, Clone, Debug, Default, Pod)]
#[repr(C)]
pub struct ClientClass {
    pub pCreateFn: Ptr,
    pub pCreateEventFn: Ptr,
    pub pNetworkName: Ptr<[u8]>,
    pub pRecvTable: Ptr<RecvTable>,
    pub pNext: Ptr<ClientClass>,
    pub ClassID: i32,
    pub ClassSize: u32,
}

#[derive(Copy, Clone, Debug, Default, Pod)]
#[repr(C)]
pub struct RecvTableRedux {
    pub inst: Ptr,
    pub props: Ptr<[Ptr<RecvProp>]>, // Goes through heap :(
    pub num_props: i32,
    pub pad: u32,
}

#[derive(Copy, Clone, Debug, Pod)]
#[repr(C)]
pub struct RecvTable {
    pub inst: Ptr,
    pub props: Ptr<[Ptr<RecvProp>]>, // Goes through heap :(
    pub num_props: i32,
    _unk0: [u32; 256],
    _unk1: [u32; 43],
    pub decoder: Ptr,
    pub name: Ptr<[u8]>,
    pub initialized: u8,
    pub in_main_list: u8,
    _unk2: [u8; 6],
}
const _: [(); 0x4D8] = [(); mem::size_of::<RecvTable>()];

#[derive(Copy, Clone, Debug, Default, Pod)]
#[repr(C)]
pub struct RecvProp {
    pub ty: i32,
    pub offset: i32,
    _unk1: [u32; 6],
    pub data_table: Ptr<RecvTable>,
    pub name: Ptr<[u8]>,
    pub is_inside_array: u8,
    _unk2: [u8; 7],
    pub array_prop: Ptr<RecvProp>,
    pub proxy_fn: Ptr,
    _unk3: [u32; 4],
    pub flags: i32,
    _unk4: u32,
    pub num_elements: i32,
    _unk5: u32,
}
const _: [(); 0x68] = [(); mem::size_of::<RecvProp>()];
