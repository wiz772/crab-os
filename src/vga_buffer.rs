use core::fmt;

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

impl ColorCode{
    const fn new(foreground: Color, background: Color) -> ColorCode{
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH : usize = 80;

#[repr(transparent)]
struct Buffer{
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer{
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}

pub static WRITER: Writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Black, Color::White),
    buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
};

impl Writer{
    pub fn write_byte(&mut self, byte: u8){
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;

                self.buffer.chars[row][col] = ScreenChar{ascii_character: byte, color_code};
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str){
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn clear_row(&mut self, row: usize){
        let color_code = self.color_code;
        let blank_character = ScreenChar{ascii_character: b' ', color_code};
        for column in 0..BUFFER_WIDTH {
            self.buffer.chars[row][column] = blank_character;
        }
    }

    pub fn clear_column(&mut self, column: usize){
        let color_code = self.color_code;
        let blank_character = ScreenChar{ascii_character: b' ', color_code};
        for row in 0..BUFFER_HEIGHT {
            self.buffer.chars[row][column] = blank_character;
        }
    }

    pub fn fill_with_char(&mut self, byte: u8){
        let color_code = self.color_code;
        for row in 0..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH  {
                self.buffer.chars[row][column] = ScreenChar{ascii_character: byte, color_code};
            }
        }
    }

    pub fn new_line(&mut self){
            for row in 1..BUFFER_HEIGHT{
                for col in 0..BUFFER_WIDTH{
                    let c = self.buffer.chars[row][col];
                    self.buffer.chars[row - 1][col] = c;
                }
            }

        self.column_position = 0;
        self.clear_row(BUFFER_HEIGHT - 1);
    }

}

impl fmt::Write for Writer{

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }

}

pub fn at_sign_filler(){
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Black, Color::White),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}
    };

    writer.fill_with_char(b'w');
}

pub fn print_something(s: &str){
    use core::fmt::Write;
    let mut writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Black, Color::White),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}

    };
    write!(writer, "{}", s).unwrap();
    
}


