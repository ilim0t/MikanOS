use super::*;
use core::{
    fmt,
    ops::{Index, IndexMut},
};

const ROW_MAX: usize = 25; // Height
const COLUMN_MAX: usize = 80; // Width

struct Cursor {
    column: usize,
    row: usize,
}

impl Cursor {
    fn new() -> Cursor {
        Cursor { column: 0, row: 0 }
    }

    fn to_pixel(&self) -> PixelPoint {
        PixelPoint {
            x: self.column * 8,
            y: self.row * 16,
        }
    }
}

struct StringBuffer {
    buffer: [[u8; COLUMN_MAX]; ROW_MAX],
}

impl Index<usize> for StringBuffer {
    type Output = [u8; COLUMN_MAX];
    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl IndexMut<usize> for StringBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

impl StringBuffer {
    fn new() -> StringBuffer {
        StringBuffer {
            buffer: [[b' '; COLUMN_MAX]; ROW_MAX],
        }
    }
}

pub struct Console {
    writer: PixelWriter,
    bg_color: Color,
    fg_color: Color,
    cursor: Cursor,
    buffer: StringBuffer,
}

impl Console {
    pub fn new(writer: PixelWriter, bg_color: Color, fg_color: Color) -> Console {
        let mut console = Console {
            writer,
            bg_color,
            fg_color,
            cursor: Cursor::new(),
            buffer: StringBuffer::new(),
        };
        console.refresh();
        console
    }

    #[allow(unreachable_patterns)]
    pub fn write_string(&mut self, strings: &str) {
        for byte in strings.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte), // ASCII
                _ => self.write_byte(b'\x02'),
                0xa1..=0xdf => self.write_byte(byte), // カタカナ等
                0x00..=0x1f => {}                     // 特殊記号
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.new_line();
        } else if self.cursor.column < COLUMN_MAX {
            self.writer
                .write_byte(&self.cursor.to_pixel(), byte, &self.fg_color);
            self.buffer[self.cursor.row][self.cursor.column] = byte;
            self.cursor.column += 1;
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_byte(byte);
        }
    }

    pub fn new_line(&mut self) {
        self.cursor.column = 0;

        if self.cursor.row < ROW_MAX - 1 {
            self.cursor.row += 1;
        } else {
            for row in 0..ROW_MAX - 1 {
                self.buffer[row] = self.buffer[row + 1];
            }
            self.buffer[ROW_MAX - 1].fill(b' ');
            self.refresh();
            self.cursor.row = ROW_MAX - 1;
        }
    }

    pub fn refresh(&mut self) {
        self.writer.clear(&self.bg_color);

        for row in 0..ROW_MAX {
            for column in 0..COLUMN_MAX {
                self.writer.write_byte(
                    &Cursor { row, column }.to_pixel(),
                    self.buffer[row][column],
                    &self.fg_color,
                );
            }
        }
    }
    pub fn font_gallery(&mut self) {
        for i in 0..=0xff {
            let row = i as usize / 20;
            let column = i as usize % 20;
            self.buffer[row][column] = i;
        }
        self.cursor = Cursor {
            row: 0xff / 20 + 1,
            column: 0,
        };
        self.refresh();
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
