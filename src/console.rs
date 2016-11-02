
use super::portio;

pub struct Console {
    video_mem: *mut [[u16; 80]; 25]
}

pub fn get_console() -> Console {
    // FIXME: This is the same object, no matter how many times we return it;
    // we should do proper synchronization (and define what that means).
    Console{video_mem: 0xb8000 as *mut [[u16; 80]; 25]}
}

impl Console {
    pub fn set_cell(&mut self, x : usize, y : usize, fg : Color, bg : Color, chr : u8) {
        self.check_bounds(x, y).unwrap();

        let (Color(fore), Color(back)) = (fg, bg);
        unsafe {
            (*self.video_mem)[y][x] = (chr as u16) | ((back << 4 | fore ) << 8);
        }
    }

    pub fn move_cursor(&mut self, x : usize, y : usize) {
        self.check_bounds(x, y).unwrap();

        // The procedure here is pulled from [molloy], mostly for the magic constants.
        let cmd_port  : u16 = 0x3d4;
        let data_port : u16 = 0x3d5;
        let set_hi : u8 = 14;
        let set_lo : u8 = 15;

        // Cast to usize to avoid overflowing u8:
        let position = (y as usize) * 80 + (x as usize);

        unsafe {
            portio::outb(cmd_port, set_hi);
            portio::outb(data_port, (position >> 8) as u8);
            portio::outb(cmd_port, set_lo);
            portio::outb(data_port, position as u8);
        }
    }

    fn check_bounds(&self, x: usize, y: usize) -> Result<(), ()> {
        if y >= 25 || x >= 80 {
            Err(())
        } else {
            Ok(())
        }
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
