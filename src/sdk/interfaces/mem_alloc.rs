use crate::oof::ptr::get_vfunc_ptr_as;

#[derive(Debug)]
pub struct MemAlloc {
    pub this_ptr: *mut u8,
}

impl MemAlloc {
    pub unsafe fn new(this: *mut u8) -> MemAlloc {
        MemAlloc { this_ptr: this }
    }

    pub unsafe fn alloc(&self, size: usize) -> *mut u8 {
        type Fn = unsafe extern "win64" fn(this: *mut u8, size: usize) -> *mut u8;
        get_vfunc_ptr_as::<Fn>(self.this_ptr, 1)(self.this_ptr, size)
    }

    pub unsafe fn realloc(&self, mem: *mut u8, new_size: usize) -> *mut u8 {
        type Fn = unsafe extern "win64" fn(this: *mut u8, mem: *mut u8, new_size: usize) -> *mut u8;
        get_vfunc_ptr_as::<Fn>(self.this_ptr, 2)(self.this_ptr, mem, new_size)
    }

    pub unsafe fn free(&self, mem: *mut u8) {
        type Fn = unsafe extern "win64" fn(this: *mut u8, mem: *mut u8);
        get_vfunc_ptr_as::<Fn>(self.this_ptr, 3)(self.this_ptr, mem)
    }

    pub unsafe fn get_size(&self, mem: *mut u8) -> usize {
        // FIXME: This returns bullshit
        type Fn = unsafe extern "win64" fn(this: *mut u8, mem: *mut u8) -> usize;
        get_vfunc_ptr_as::<Fn>(self.this_ptr, 21)(self.this_ptr, mem)
    }
}
