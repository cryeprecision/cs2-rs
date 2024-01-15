// TODO: Implement https://github.com/cryeprecision/megafaggot_cmake/blob/main/oof/include/oof/memory.h but for x64

pub unsafe fn follow_jmp(_start: *const u8) -> Option<*const u8> {
    todo!("https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/utilities/memory.h#L74");
}

pub unsafe fn follow_jmp_mut(_start: *mut u8) -> Option<*mut u8> {
    todo!("https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/utilities/memory.h#L74");
}

pub unsafe fn get_vfunc_ptr(this: *const u8, index: usize) -> *const u8 {
    // the first 'member' of the object is the vtable pointer
    let vtable_ptr = *(this as *const *const u8);
    // get the index-th function-pointer in the vtable
    *(vtable_ptr.add(index) as *const *const u8)
}
