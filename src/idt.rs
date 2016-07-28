

pub struct Gate {
  offset : u64,
  selector : u16,
  present : bool,
  ist : u8, // Interrupt stack table. 3 bits
  typ : u8, // 4 bits
  dpl : u8, // Discriptor priviledge level. 2 bits.
}

fn bool2bit(b : bool) -> u8 {
  match b {
    true => 1,
    false => 0
  }
}

impl Gate {
  pub fn marshal(&self) -> [u8; 16] {
    [
      (self.offset & 0xff) as u8,
      ((self.offset >> 8) & 0xff) as u8,
      (self.selector & 0xff) as u8,
      ((self.selector >> 8) & 0xff) as u8,
      (self.ist & 0x7),
      (self.typ & 0xf) | (self.dpl<<5) | bool2bit(self.present),
      ((self.offset >> 16) & 0xff) as u8,
      ((self.offset >> 24) & 0xff) as u8,
      ((self.offset >> 32) & 0xff) as u8,
      ((self.offset >> 40) & 0xff) as u8,
      ((self.offset >> 48) & 0xff) as u8,
      ((self.offset >> 56) & 0xff) as u8,
      0,0,0,0, // reserved
    ]
  }

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
