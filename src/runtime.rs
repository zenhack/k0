/* Basic symbol declarations and such needed by the rust runtime. */

extern crate rlibc;

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop {}}

// vim: set ts=2 sw=2 et :
