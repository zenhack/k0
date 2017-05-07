
use super::console;
use super::console::{get_console, RED, GREEN, BLACK};
use super::serial;
use super::bochs;
use core::fmt::Write;

#[no_mangle]
pub extern fn bsp_main() {
    let mut console = unsafe { get_console() };
    console.set_cell(4, 2, GREEN, BLACK, '!' as u8);
    console.move_cursor(0, 0);
    serial::init(serial::COM1);
    serial::COM1.write_str("Hello\n").unwrap();
    let mut w = console::Writer::from_console(console, 0, 0, RED, BLACK);
    unsafe { bochs::breakpoint() };

    // Note to self: this is causing triple-faults with my toolchain.
    // the reason is because rustc happens to generate SSE instructions
    // with it (and doesn't if we leave it out). I need to set up a
    // proper toolchain before I continue.
    //
    // The thing below is likely related.
    w.write_str("Hello\n").unwrap()

    // This is currently expanding to something that calls
    // ::std::somethingsomething, and moreover it's
    // tripe-faulting the machine. I'm going to leave this
    // commented out and not use writeln! and friends until
    // I understand how they interact with bare-metal.
    //  writeln!(serial::COM1, "Hello, World!").unwrap()
}
