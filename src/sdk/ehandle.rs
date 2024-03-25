use super::{Pod, NUM_ENT_ENTRIES};
use std::fmt;

#[derive(Pod, Copy, Clone, Eq)]
#[repr(transparent)]
pub struct EHandle(u32);
impl Default for EHandle {
    fn default() -> EHandle {
        EHandle(!0)
    }
}
impl fmt::Debug for EHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmtools::write!(f, "EHandle("{self.index():?}")")
    }
}
impl From<i32> for EHandle {
    fn from(index: i32) -> EHandle {
        EHandle(index as u32)
    }
}
impl From<u32> for EHandle {
    fn from(raw: u32) -> EHandle {
        EHandle(raw)
    }
}
impl From<EHandle> for u32 {
    fn from(ehandle: EHandle) -> u32 {
        ehandle.0
    }
}
impl EHandle {
    pub fn is_valid(self) -> bool {
        let Self(handle) = self;
        handle != !0
    }
    pub fn index(self) -> Option<usize> {
        let Self(handle) = self;
        if handle == !0 {
            None
        } else {
            Some(handle as usize & (NUM_ENT_ENTRIES - 1))
        }
    }
    pub fn signed_index(self) -> i32 {
        let Self(handle) = self;
        if handle == !0 {
            -1
        } else {
            (handle as usize & (NUM_ENT_ENTRIES - 1)) as i32
        }
    }
}
impl PartialEq for EHandle {
    fn eq(&self, other: &Self) -> bool {
        self.signed_index() == other.signed_index()
    }
}
