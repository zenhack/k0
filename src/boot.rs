
use super::console;
use super::serial;
use core::fmt::Write;

#[no_mangle]
pub extern fn bsp_main() {
  console::set_cell(4, 2, console::GREEN, console::BLACK, '!' as u8);
  console::move_cursor(0, 0);
  serial::init(serial::COM1);
  serial::COM1.write_str("Hello\n").unwrap()
}

// vim: set ts=2 sw=2 et :
