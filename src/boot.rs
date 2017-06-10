
use super::console::{Console, RED, GREEN, BLACK};
use super::serial;
use super::bochs;
use super::util::fmt::MultiWriter;
use super::idt;
use core::fmt::Write;

#[no_mangle]
pub extern fn bsp_main() {
    let mut console = unsafe { Console::get_global() };
    console.clear(BLACK);
    console.set_cell(4, 2, GREEN, BLACK, '!' as u8);
    serial::init(serial::COM1);
    let mut w = MultiWriter::new(
        serial::COM1,
        console.to_writer(0, 0, RED, BLACK)
    );
    writeln!(w, "Hello, World!").unwrap();
    unsafe { idt::init(); }
    unsafe { asm!("int $$0x7"); }
    writeln!(w, "Returned from interrupt.");
}
