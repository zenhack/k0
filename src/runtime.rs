/* Basic symbol declarations and such needed by the rust runtime. */

extern crate rlibc;

use core::fmt;

#[lang = "eh_personality"]
extern fn rust_eh_personality() {}

#[no_mangle]
#[allow(unused_variables)]
#[lang = "panic_fmt"]
pub extern fn rust_begin_panic(args : fmt::Arguments, file : &str, line : u32) -> ! {loop{}}
