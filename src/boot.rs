
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
    let mut w = console::Writer::from_console(console, 0, 0, RED, BLACK);

    writeln!(serial::COM1, "Hello, World!").unwrap();
    writeln!(w, "Hello, World!").unwrap()
}
