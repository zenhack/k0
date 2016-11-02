
use super::console;
use super::serial;
use core::fmt::Write;

#[no_mangle]
pub extern fn bsp_main() {
  console::get_console().set_cell(4, 2, console::GREEN, console::BLACK, '!' as u8);
  console::get_console().move_cursor(0, 0);
  serial::init(serial::COM1);
  serial::COM1.write_str("Hello\n").unwrap()

    // This is currently expanding to something that calls
    // ::std::somethingsomething, and moreover it's
    // tripe-faulting the machine. I'm going to leave this
    // commented out and not use writeln! and friends until
    // I understand how they interact with bare-metal.
//  writeln!(serial::COM1, "Hello, World!").unwrap()
}

// vim: set ts=2 sw=2 et :
