
use super::bochs;

use super::portio;
use core::fmt;

const NUM_COLS: usize = 80;
const NUM_ROWS: usize = 25;

pub struct Console {}

unsafe fn video_mem() -> *mut [[u16; NUM_COLS]; NUM_ROWS] {
    0xb8000 as *mut [[u16; NUM_COLS]; NUM_ROWS]
}

/// A reference to the x86 text video console.
impl Console {

    /// Returns a reference to the console.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to make sure use of this function
    /// never results in more than one live Console object.
    pub unsafe fn get_global() -> Self {
        // This is just a phantom object; the address is always fixed, so the struct
        // needn't contain any information. It's just there so that we can
        // centralize the use of `unsafe` in the exposed api.
        Console{}
    }

    /// Sets the contents of a cell on the screen.
    ///
    /// The cell at position `(x, y)` is set to the character `chr`, with a
    /// foreground color `fg` and background color `bg`.
    pub fn set_cell(&mut self, x : usize, y : usize, fg : Color, bg : Color, chr : u8) {
        self.check_bounds(x, y).unwrap();

        let (Color(fore), Color(back)) = (fg, bg);
        unsafe {
            (*video_mem())[y][x] = (chr as u16) | ((back << 4 | fore ) << 8);
        }
    }

    /// Move the hardware cursor to position `(x, y)`.
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

    /// Clears the screen
    ///
    /// The background will be the color `bg`.
    pub fn clear(&mut self, bg: Color) {
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                // Foreground color is arbitrary; doesn't matter:
                self.set_cell(x, y, WHITE, bg, ' ' as u8);
            }
        }
    }

    /// Consume the console, converting it to a writer starting at the screen
    /// position (x,y), and using the given foreground and background color.
    pub fn to_writer(mut self, x: usize, y: usize, fg: Color, bg: Color) -> Writer {
        self.move_cursor(x, y);
        Writer{
            console: self,
            fg: fg,
            bg: bg,
            x: x,
            y: y
        }
    }

    fn check_bounds(&mut self, x: usize, y: usize) -> Result<(), ()> {
        if y >= NUM_ROWS || x >= NUM_COLS {
            Err(())
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Copy, Debug)]
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

/// An implementation of core::std::Write on top of a Console.
pub struct Writer {
    console: Console,
    fg: Color,
    bg: Color,
    x: usize,
    y: usize
}

impl Writer {

    /// Cosnume the writer, returning the underlying Console reference.
    pub fn to_console(self) -> Console { self.console }

    pub fn putc(&mut self, byte: u8) {
        match byte as char {
            c if c >= ' ' && c <= '~' => {
                // printable char.
                self.console.set_cell(self.x, self.y, self.fg, self.bg, byte);
                self.x += 1;
            }
            '\n' => {
                self.x = 0;
                self.y += 1;
            }
            '\t' => {
                // move to the next tab stop:
                while {
                    self.putc(' ' as u8);
                    self.x % 8 != 0
                } { /* Note that this is a do-while style loop. */ }
            }
            _ => {
                // some other non-printing character; ignore it.
            }
        }
        if self.x >= NUM_COLS {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= NUM_ROWS {
            self.scroll();
        }
        self.console.move_cursor(self.x, self.y);
    }

    fn scroll(&mut self) {
        for y in 0..NUM_ROWS-1 {
            for x in 0..NUM_COLS {
                unsafe {
                    let arr = video_mem();
                    (*arr)[x][y] = (*arr)[x][y+1];
                }
            }
        }
        for x in 0..NUM_COLS {
            self.console.set_cell(x, NUM_ROWS-1, self.fg, self.bg, ' ' as u8);
        }
        self.x = 0;
        self.y = NUM_ROWS - 1;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.putc(b);
        }
        Ok(())
    }
}
