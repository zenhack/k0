

#[repr(C)]
pub struct Gate {
    bytes: [u8; 16]
}


#[repr(C,packed)]
pub struct IDTPtr {
    limit: u16,
    offset: u64
}

unsafe fn lidt(idtptr: *mut IDTPtr) {
    asm!("lidt (%rax)" :: "{rax}"(idtptr) :: "volatile");
}

impl Gate {

    pub fn new(
        offset: u64,
        selector: u16,
        present : bool,
        ist : u8, // Interrupt stack table. 3 bits
        typ : u8, // 4 bits
        dpl : u8, // Discriptor priviledge level. 2 bits.
    ) -> Gate {
        Gate{bytes: [
          (offset & 0xff) as u8,
          ((offset >> 8) & 0xff) as u8,
          (selector & 0xff) as u8,
          ((selector >> 8) & 0xff) as u8,
          (ist & 0x7),
          (typ & 0xf) | (dpl<<5) | bool2bit(present),
          ((offset >> 16) & 0xff) as u8,
          ((offset >> 24) & 0xff) as u8,
          ((offset >> 32) & 0xff) as u8,
          ((offset >> 40) & 0xff) as u8,
          ((offset >> 48) & 0xff) as u8,
          ((offset >> 56) & 0xff) as u8,
          0,0,0,0, // reserved
        ]}
    }
}

fn bool2bit(b : bool) -> u8 {
  match b {
    true => 1,
    false => 0
  }
}


/*
impl Gate {

  pub fn unmarshal(&mut self, buf : &[u8; 16]) {
    self.offset =
      ( buf[0]  as u64)        |
      ((buf[1]  as u64) << 8)  |
      ((buf[6]  as u64) << 16) |
      ((buf[7]  as u64) << 24) |
      ((buf[8]  as u64) << 32) |
      ((buf[9]  as u64) << 40) |
      ((buf[10] as u64) << 48) |
      ((buf[11] as u64) << 56) ;
    self.selector = (buf[2] as u16) | ((buf[3] as u16) << 8);
    self.ist = buf[4] & 0x7;
    self.typ = buf[5] & 0xf;
    self.dpl = (buf[5]>>5) & 0x3;
    self.present = (buf[5] & (1<<7)) != 0;
  }
}
*/
