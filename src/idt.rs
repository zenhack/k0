use super::idt_common::{Gate, BOOT_IDT, NUM_IDT_ENTS};
use super::idt_gen::init_boot_idt;
use core::mem::size_of;

#[repr(C,packed)]
struct IDTPtr {
    limit: u16,
    offset: *const [Gate; NUM_IDT_ENTS],
}

static mut IDT_PTR: IDTPtr = IDTPtr{
    limit: 0,
    offset: 0 as *const [Gate; NUM_IDT_ENTS],
};

unsafe fn lidt(idtptr: *const IDTPtr) {
    asm!("lidt (%rax)" :: "{rax}"(idtptr) :: "volatile");
}

pub unsafe fn init() {
    IDT_PTR = IDTPtr{
        limit: (NUM_IDT_ENTS * size_of::<Gate>() - 1) as u16,
        offset: &BOOT_IDT as *const [Gate; NUM_IDT_ENTS],
    };

    init_boot_idt();
    lidt(&IDT_PTR);
}
