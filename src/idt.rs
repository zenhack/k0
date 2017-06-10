use super::idt_common::{Gate, BOOT_IDT, NUM_IDT_ENTS};
use super::idt_gen::init_boot_idt;
use core::mem::size_of;

#[repr(C,packed)]
pub struct IsrSave {
	r11: u64,
	r10: u64,
	r9: u64,
	r8: u64,
	rcx: u64,
	rdx: u64,
	rsi: u64,
	rdi: u64,
	rax: u64,
    int_no: u64,
    err_code: u64,
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

fn no_int_handler(state: &mut IsrSave) {
    panic!("No interrupt handler for interrupt #{}!", state.int_no)
}

static mut INT_HANDLERS: [fn(&mut IsrSave) -> (); NUM_IDT_ENTS] =
    [no_int_handler; NUM_IDT_ENTS];


#[no_mangle]
pub unsafe extern fn int_handler_main(state: &mut IsrSave) {
    INT_HANDLERS[state.int_no as usize](state)
}

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

fn int_handler_noop(state: &mut IsrSave) {

}

pub unsafe fn init() {
    INT_HANDLERS[7] = int_handler_noop;

    IDT_PTR = IDTPtr{
        limit: (NUM_IDT_ENTS * size_of::<Gate>() - 1) as u16,
        offset: &BOOT_IDT as *const [Gate; NUM_IDT_ENTS],
    };

    init_boot_idt();
    lidt(&IDT_PTR);
}
