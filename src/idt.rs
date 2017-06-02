




#[repr(C,packed)]
#[derive(Clone, Copy)]
pub struct Gate {
    offset_lo: u16,
    segment: u16,
    ist: u8, // Only the bottom 3 bits are significant; rest must be zero.
    type_dpl_p: u8, // three fields on one byte, of sizes 4, 2, and 2 bits
                    // respectively. There is a zero bit before the dpl.
    offset_mid: u16,
    offset_hi: u32,
}


#[repr(C,packed)]
pub struct IDTPtr {
    limit: u16,
    offset: u64
}


// A zeroed-out Gate struct, which we use in a couple places. We can't just
// derive Default, since we use it in the top-level definition boot_idt.
const ZERO_GATE: Gate = Gate{
    offset_lo: 0,
    segment: 0,
    ist: 0,
    type_dpl_p: 0,
    offset_mid: 0,
    offset_hi: 0,
};

static boot_idt: [Gate; 256] = [ZERO_GATE; 256];

unsafe fn lidt(idtptr: *const IDTPtr) {
    asm!("lidt (%rax)" :: "{rax}"(idtptr) :: "volatile");
}

impl Gate {

    pub fn new(
        offset: u64,
        segment: u16,
        present : bool,
        ist : u8, // Interrupt stack table. 3 bits
        typ : u8, // 4 bits
        dpl : u8, // Discriptor priviledge level. 2 bits.
    ) -> Gate {
        let mut ret = ZERO_GATE;
        ret.set_offset(offset);
        ret.set_segment(segment);
        ret.set_present(present);
        ret.set_dpl(dpl);
        return ret;
    }

    pub fn set_offset(&mut self, offset: u64) {
        self.offset_lo  = ((offset >> 0 ) & 0xffff    ) as u16;
        self.offset_mid = ((offset >> 16) & 0xffff    ) as u16;
        self.offset_hi  = ((offset >> 32) & 0xffffffff) as u32;
    }

    pub fn get_offset(&self) -> u64 {
        return
            ((self.offset_lo  as u64) << 0 ) |
            ((self.offset_mid as u64) << 16) |
            ((self.offset_hi  as u64) << 32) ;
    }

    pub fn set_present(&mut self, present: bool) {
        let present_bit = if present { 1 } else { 0 };
        self.type_dpl_p &= !(1 << 7);
        self.type_dpl_p |= present_bit << 7;
    }

    pub fn get_present(&self) -> bool {
        self.type_dpl_p & (1<<7) != 0
    }

    pub fn set_dpl(&mut self, dpl: u8) {
        if dpl > 3 {
            panic!("DPL out of range");
        }
        self.type_dpl_p &= 3<<5;
        self.type_dpl_p |= dpl << 5;
    }

    pub fn get_dpl(&self) -> u8 {
        (self.type_dpl_p >> 5) & 3
    }

    pub fn set_segment(&mut self, segment: u16) {
        self.segment = segment;
    }

    pub fn get_segment(&self) -> u16 {
        self.segment
    }

}
