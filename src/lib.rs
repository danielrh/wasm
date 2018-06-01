extern crate core;
pub mod ffi;


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
