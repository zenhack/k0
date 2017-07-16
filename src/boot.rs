
use super::console::{Console, LIGHT_GREY, BLACK};
use super::serial;
use super::util::fmt::MultiWriter;
use super::idt;
use core::fmt::Write;

#[no_mangle]
pub extern fn bsp_main() {
    unsafe { idt::init(); }

    let mut console = unsafe { Console::get_global() };
    console.clear(BLACK);
    serial::init(serial::COM1);
    let mut w = MultiWriter::new(
        serial::COM1,
        console.to_writer(0, 0, LIGHT_GREY, BLACK)
    );

    writeln!(w, "Booting k0 (pre-alpha)...").unwrap();
}
