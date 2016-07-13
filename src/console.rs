
pub fn set_cell(x : usize, y : usize, fg : Color, bg : Color, chr : u8) {
  // XXX: this is declared locally instead of static to get around
  // having to declare it `Sync`, but it has the same effect; we're not
  // doing any synchronization here. We should be deliberate about what
  // the semantics of this are re: concurrency, but right now we're just
  // saying "screw it."
  let video_mem = 0xb8000 as *mut [[u16; 80]; 25];

  let (Color(fore), Color(back)) = (fg, bg);
  unsafe {
    (*video_mem)[y][x] = (chr as u16) | ((back << 4 | fore ) << 8);
  }
}

pub struct Color(u16);

pub const BLACK         : Color = Color(0x0);
pub const BLUE          : Color = Color(0x1);
pub const GREEN         : Color = Color(0x2);
pub const CYAN          : Color = Color(0x3);
pub const RED           : Color = Color(0x4);
pub const MAGENTA       : Color = Color(0x5);
pub const BROWN         : Color = Color(0x6);
pub const LIGHT_GREY    : Color = Color(0x7);
pub const DARK_GREY     : Color = Color(0x8);
pub const LIGHT_BLUE    : Color = Color(0x9);
pub const LIGHT_GREEN   : Color = Color(0xa);
pub const LIGHT_CYAN    : Color = Color(0xb);
pub const LIGHT_RED     : Color = Color(0xc);
pub const LIGHT_MAGENTA : Color = Color(0xd);
pub const LIGHT_BROWN   : Color = Color(0xe);
pub const WHITE         : Color = Color(0xf);
