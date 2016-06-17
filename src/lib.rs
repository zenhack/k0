#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub extern fn bsp_main() {
  let video_mem = 0xb8000 as *mut [[u16; 25]; 80];
  let black = 0x0;
  let light_green = 0xa;
  let ch = '!' as u16;
  unsafe {
    (*video_mem)[0][0] = ch | ((black << 4 | light_green) << 8);
  }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop {}}


#[no_mangle]
pub unsafe extern "C" fn memset(buf: *mut u8, value: u8, size: usize) -> *mut u8 {
  let mut i = 0;
  while i < size {
    *buf.offset(i as isize) = value;
    i += 1;
  }
  return buf
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, size: usize) {
  let mut i = 0;
  while i < size {
    *dest.offset(i as isize) = *src.offset(i as isize);
    i += 1;
  }
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, size: usize) -> i32 {
  let mut i: isize = 0;
  while (i as usize) < size {
    if *s1.offset(i) < *s2.offset(i) {
      return -1
    } else if *s1.offset(i) > *s2.offset(i) {
      return 1
    } else {
      i += 1
    }
  }
  return 0
}


// TODO: not sure these implementations are correct, particularly wrt error
// handling. fmod(3) says they set errno, but that's an extra dep...
#[no_mangle]
pub unsafe extern "C" fn fmod(x: f64, y: f64) -> f64 {
  let n = x / y;
  return x - n * y;
}

#[no_mangle]
pub unsafe extern "C" fn fmodf(x: f32, y: f32) -> f32 {
  let n = x / y;
  return x - n * y;
}

// vim: set ts=2 sw=2 et :
