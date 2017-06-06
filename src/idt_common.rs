//! IDT-related code needed both by idt_gen and the top-level idt module.

pub const NUM_IDT_ENTS: usize = 256;

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
    reserved: u32,
}

pub const ZERO_GATE: Gate = Gate{
    offset_lo: 0,
    segment: 0,
    ist: 0,
    type_dpl_p: 0,
    offset_mid: 0,
    offset_hi: 0,
    reserved: 0,
};


pub static mut BOOT_IDT: [Gate; NUM_IDT_ENTS] = [ZERO_GATE; NUM_IDT_ENTS];

impl Gate {

    /// Create a new interrupt gate.
    ///
    /// `handler` is the interrupt handler. Note that these to *not* follow
    /// the same calling convention as rust and/or C -- they must be defined
    /// in assembly. See `isr.s` and `make_isrs.sh`.
    ///
    /// the other fields correspond to what is documented in the intel manual.
    ///
    /// Note that "type" is always that of a 64-bit interrupt gate.
    pub fn new(
        handler: unsafe extern fn() -> (),
        segment: u16,
        present : bool,
        ist : u8, // Interrupt stack table. 3 bits
        dpl : u8, // Discriptor priviledge level. 2 bits.
    ) -> Gate {
        let mut ret = ZERO_GATE;
        ret.set_offset(handler as u64);
        ret.set_segment(segment);
        ret.set_present(present);
        ret.set_dpl(dpl);

        ret.set_type(0b1110); // [intel/3/3.5] (table 3-2).
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
        self.type_dpl_p &= !(3 << 5);
        self.type_dpl_p |= dpl << 5;
    }

    pub fn get_dpl(&self) -> u8 {
        (self.type_dpl_p >> 5) & 3
    }

    fn set_type(&mut self, typ: u8) {
        if typ > 0xf {
            panic!("descriptor type is too large (max 4 bits)");
        }
        self.type_dpl_p &= 0xf0;
        self.type_dpl_p |= typ;
    }

    fn get_type(&mut self) -> u8 {
        self.type_dpl_p & 0xf
    }

    pub fn set_segment(&mut self, segment: u16) {
        self.segment = segment;
    }

    pub fn get_segment(&self) -> u16 {
        self.segment
    }
}
