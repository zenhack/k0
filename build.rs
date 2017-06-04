use std::io::Write;
use std::fs::File;
use std::env;
use std::path::Path;

fn write_to<W: Write>(w: &mut W) {
    w.write_all(b"use super::idt::{Gate, ZERO_GATE};\n").unwrap();
    w.write_all(b"static mut BOOT_IDT: [Gate; 256] = [ZERO_GATE; 256];\n").unwrap();
    w.write_all(b"extern {\n").unwrap();
    for i in 0..256 {
        writeln!(w, "    #[link(name = \"isr{0}\")] fn isr{0} () -> ();", i).unwrap();
    }
    w.write_all(b"}\n").unwrap();
    w.write_all(b"pub unsafe fn init_boot_idt() {\n").unwrap();
    for i in 0..256 {
        writeln!(w, "    BOOT_IDT[{0}] = Gate::new(isr{0}, 0x8, true, 0, 0);", i)
            .unwrap();
    }
    w.write_all(b"}\n").unwrap();
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("idt_gen.rs");
    let mut file = File::create(&dest_path).unwrap();
    write_to(&mut file);
}
