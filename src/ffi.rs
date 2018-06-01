use core;
#[no_mangle]
pub unsafe extern fn wasm_alloc_u8(size: usize) -> *mut u8 {
    let mut newly_allocated = vec![0u8;size].into_boxed_slice();
    let slice_ptr = newly_allocated.as_mut_ptr();
    let _box_ptr = Box::into_raw(newly_allocated);
    slice_ptr
}



#[no_mangle]
pub unsafe extern fn wasm_free_u8(ptr: *mut u8, size: usize) {
    let slice_ref = core::slice::from_raw_parts_mut(ptr, size);
    Box::from_raw(slice_ref); // free on drop
}




#[no_mangle]
pub unsafe extern fn wasm_checkerboard(ptr: *mut u8, size: usize,
                                       offset: i32, stride: i32) -> usize {
    super::checkerboard(core::slice::from_raw_parts_mut(ptr, size), offset, stride)
}
