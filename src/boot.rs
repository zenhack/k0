
use super::console;
use super::serial;

#[no_mangle]
pub extern fn bsp_main() {
  console::set_cell(4, 2, console::GREEN, console::BLACK, '!' as u8);
  console::move_cursor(0, 0);
  serial::init(serial::COM1);
  serial::putc(serial::COM1, '*' as u8);
}

// vim: set ts=2 sw=2 et :
