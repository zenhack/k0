
use super::console;
use super::console::{get_console, RED, GREEN, BLACK};
use super::serial;
use super::bochs;
use super::util::fmt::MultiWriter;
use core::fmt::Write;

#[no_mangle]
pub extern fn bsp_main() {
    let mut console = unsafe { get_console() };
    console.clear(BLACK);
    console.set_cell(4, 2, GREEN, BLACK, '!' as u8);
    console.move_cursor(0, 0);
    serial::init(serial::COM1);
    let mut w = MultiWriter::new(
        serial::COM1,
        console::Writer::from_console(console, 0, 0, RED, BLACK)
    );
    writeln!(w, "Hello, World!").unwrap()
}
