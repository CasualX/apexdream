use super::*;

const SIZE: usize = sdk::MAX_PLAYERS * 2;

pub struct NameList {
    pointers1: Box<[sdk::Ptr<[u8]>]>,
    pointers2: Box<[sdk::Ptr<[u8]>]>,
    names: Box<[String]>,
}
impl Default for NameList {
    fn default() -> NameList {
        NameList {
            pointers1: vec![sdk::Ptr::new(); SIZE].into_boxed_slice(),
            pointers2: vec![sdk::Ptr::new(); SIZE].into_boxed_slice(),
            names: vec![String::new(); SIZE].into_boxed_slice(),
        }
    }
}
impl NameList {
    pub fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
        let process = ctx.process;
        let data = ctx.data;

        if !ctx.ticked(25, 19) {
            return;
        }

        // Read the name list and check for new names
        let _ = api.vm_read_into(process.base.field(data.name_list), &mut *self.pointers2);
        std::mem::swap(&mut self.pointers1, &mut self.pointers2);

        let pointers1 = unsafe { self.pointers1.get_unchecked(..SIZE) };
        let pointers2 = unsafe { self.pointers2.get_unchecked(..SIZE) };
        let names = unsafe { self.names.get_unchecked_mut(..SIZE) };
        let mut name_buf = [0u8; 128];
        for i in 0..SIZE {
            if pointers1[i] != pointers2[i] {
                names[i].clear();
                if let Ok(name) = api.vm_read_cstr(pointers1[i], &mut name_buf) {
                    names[i].push_str(name);
                }
            }
        }
    }
}
impl GameState {
    pub fn get_player_name(&self, handle: sdk::EHandle) -> Option<&str> {
        let index = handle.index()?.wrapping_sub(1).wrapping_mul(2);
        let name = self.name_list.names.get(index)?;
        Some(name.as_str())
    }
    pub fn get_name1(&self, index: usize) -> &str {
        match self.name_list.names.get(index * 2) {
            Some(name) => &**name,
            None => "",
        }
    }
    pub fn get_name2(&self, index: usize) -> &str {
        match self.name_list.names.get(index * 2 + 1) {
            Some(name) => &**name,
            None => "",
        }
    }
}
