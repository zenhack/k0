/* Basic symbol declarations and such needed by the rust runtime. */

extern crate rlibc;

use core::fmt;
use core::fmt::Write;
use super::console;

#[lang = "eh_personality"]
extern fn rust_eh_personality() {}

#[no_mangle]
#[allow(unused_variables)]
#[lang = "panic_fmt"]
pub extern fn rust_begin_panic(args : fmt::Arguments, file : &str, line : u32) -> ! {
    // It's possible this will result in multiple references to the console.
    // However, the worst that can actually happen is we get some garbled
    // screen output; for handling a panic it seems worth the risk.
    let mut w = unsafe { console::Console::get_global() }
        .to_writer(0, 0, console::WHITE, console::RED);

    // Not much we can do if these fail, but we can't .unwrap() either, since
    // we're already panicing. Furthermore, the console never actually fails
    // here. just ignore the Results:
    let _ = write!(w, "PANIC at {} line {}: ", file, line);
    let _ = w.write_fmt(args);

    // And of course, panic is not allowed to return.
    loop { unsafe { asm!("hlt") } }
}
