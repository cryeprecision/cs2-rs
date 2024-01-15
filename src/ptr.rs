/// # Arguments
///
/// * `rva_offset` - offset of the relative address
/// * `rip_offset` - offset of the instruction pointer
pub unsafe fn resolve_relative_address(
    addr: *const u8,
    rva_offset: isize,
    rip_offset: isize,
) -> *const u8 {
    // https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/utilities/memory.h#L74
    let rva = addr.offset(rva_offset).cast::<u32>().read();
    addr.add(rva as usize).offset(rip_offset)
}

pub unsafe fn get_vfunc_ptr(this: *const u8, index: usize) -> *const u8 {
    // 'this' is essentially a pointer to the vtable pointer, the first read results
    // in a pointer to the first vfunc pointer, we add `index` to get to the
    // index-th vfunc pointer and read that
    (this as *const *const *const u8).read().add(index).read()
}

pub unsafe fn get_vfunc_ptr_as<F>(this: *const u8, index: usize) -> F {
    // 'this' is essentially a pointer to the vtable pointer, the first read results
    // in a pointer to the first vfunc pointer, we add `index` to get to the
    // index-th vfunc pointer and read that
    (this as *const *const F).read().add(index).read()
}
