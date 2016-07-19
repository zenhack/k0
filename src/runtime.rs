/* Basic symbol declarations and such needed by the rust runtime. */

extern crate rlibc;

//use core::fmt;

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop {}}

// #[allow(unused_variables)]
// pub extern fn rust_begin_unwind(args : fmt::Arguments, file : &str, line : u32) -> ! {loop{}}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn _Unwind_Resume() -> ! {loop{}}
// vim: set ts=2 sw=2 et :
