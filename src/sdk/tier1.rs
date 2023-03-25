use super::{Pod, Ptr};

#[derive(Debug, Default)]
#[repr(C)]
pub struct CUtlMemory<T> {
	pub pMemory: Ptr<[T]>,
	pub nAllocationCount: i32,
	pub nGrowSize: i32,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct CUtlVector<T> {
	pub Memory: CUtlMemory<T>,
	pub Size: i32,
	pub pElements: Ptr<[T]>,
}

unsafe impl<T: 'static> Pod for CUtlMemory<T> {}
unsafe impl<T: 'static> Pod for CUtlVector<T> {}
