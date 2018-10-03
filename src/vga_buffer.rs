
const BUFFER_WIDTH : usize = 25;
const BUFFER_HEIGHT : usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(fg : Color , bg : Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char : u8,
    color_code : ColorCode,
}

struct Buffer {
    chars : [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    col_pos : usize,
    color_code : ColorCode,
    buffer : &'static mut Buffer,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            col_pos: 0,
            color_code: ColorCode::new(Color::White, Color::Green),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    pub fn write_byte(&mut self, byte : u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.col_pos;

                let cc = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_char : byte,
                    color_code : cc,
                };
                self.col_pos += 1;
            },
        }
    }

    pub fn write_string(&mut self, s : &str) {
        for b in s.bytes() {
            match b {
                0x20...0x7e | b'\n' => self.write_byte(b),
                _ => self.write_byte(0xfe)
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let c = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = c;
            }
            self.clear_row(BUFFER_HEIGHT - 1);
            self.col_pos = 0;
        }
    }

    fn clear_row(&mut self, row : usize) {
        let blank = ScreenChar {
            ascii_char : b' ',
            color_code : self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }
}
