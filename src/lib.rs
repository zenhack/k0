#![feature(lang_items)]
#![no_std]

extern crate rlibc;

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

// vim: set ts=2 sw=2 et :
