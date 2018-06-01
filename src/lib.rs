extern crate core;


#[no_mangle]
pub unsafe extern fn wasm_free_u8(ptr: *mut u8, size: usize) {
    let slice_ref = core::slice::from_raw_parts_mut(ptr, size);
    Box::from_raw(slice_ref); // free on drop
}

#[no_mangle]
pub unsafe extern fn wasm_alloc_u8(size: usize) -> *mut u8 {
    let mut newly_allocated = vec![0u8;size].into_boxed_slice();
    let slice_ptr = newly_allocated.as_mut_ptr();
    let _box_ptr = Box::into_raw(newly_allocated);
    slice_ptr
}

#[no_mangle]
pub unsafe extern fn wasm_checkerboard(ptr: *mut u8, size: usize,
                                       offset: i32, stride: i32) -> usize {
    checkerboard(core::slice::from_raw_parts_mut(ptr, size), offset, stride)
}

pub fn checkerboard(img: &mut [u8], offset: i32, stride: i32) -> usize {
    let mut count = offset;
    let mut ret = 0usize;
    for pix in img.iter_mut() {
        count &= -((count != stride) as i32);
        if count == 0 {
           *pix = 0xff;
           ret += 1;
        }
        count += 1;
    }
    ret
}
