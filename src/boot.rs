
use super::console::{Console, LIGHT_GREY, BLACK};
use super::serial;
use super::util::fmt::MultiWriter;
use super::idt;
use super::multiboot;
use super::paging;
use core::fmt::Write;
use core::mem;

#[no_mangle]
pub extern fn bsp_main(mboot: *const multiboot::Info) {
    unsafe { idt::init(); }

    let mut console = unsafe { Console::get_global() };
    console.clear(BLACK);
    serial::init(serial::COM1);
    let mut w = MultiWriter::new(
        serial::COM1,
        console.to_writer(0, 0, LIGHT_GREY, BLACK)
    );

    writeln!(w, "Booting k0 (pre-alpha)...").unwrap();
    let mboot_info = unsafe { *mboot };

    writeln!(w, "mboot info structure address: 0x{:x}", mboot as usize).unwrap();

    writeln!(w, "Multiboot info structure: {:?}", mboot_info).unwrap();

    match mboot_info.mem_info() {
        None => writeln!(w, "No mem_* fields!").unwrap(),
        Some((lo, hi)) => {
            writeln!(w, "Low memory  : 0x{:x}", lo).unwrap();
            writeln!(w, "High memory : 0x{:x}", hi).unwrap();
        }
    }

    match mboot_info.mmap() {
        None => writeln!(w, "No memory map info").unwrap(),
        Some(m) => for ent in m.entries() {
            writeln!(w, "Memory map entry: {:?}", ent).unwrap();
        }
    }

    let boot_pml4 : &'static mut paging::PgStruct = unsafe {
        mem::transmute::<*mut paging::PgStruct, _>(paging::get_cr3().pml4_addr())
    };
    writeln!(w, "PML4: {:?}", boot_pml4).unwrap()
}
