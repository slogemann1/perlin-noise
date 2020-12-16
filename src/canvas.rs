use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/binding/my_exports.js")]
extern {
    #[wasm_bindgen]
    fn new_canvas(s: &str, width: u32, height: u32);

    #[wasm_bindgen]
    fn flush_canvas(s: &str, data: &[u8]);
}

pub const BLACK: Color = Color { r: 0, b: 0, g: 0, a: 255 };

pub struct Canvas {
    name: String,
    width: u32,
    height: u32,
    data: Vec<u8>
}

pub struct Color {
    r: u8,
    b: u8,
    g: u8,
    a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: 255
        }
    }

    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a
        }
    }
}

impl Canvas {
    pub fn new(width: u32, height: u32, name: &str) -> Canvas {
        let mut data: Vec<u8> = Vec::new();
        for i in 0..(width * height * 4) {
            data.push(0);
        }

        new_canvas(name, width, height);

        Canvas {
            name: name.to_string(),
            width: width,
            height: height,
            data: data
        }
    }

    pub fn pixel(&mut self, x: i32, y: i32, c: Color) {
        let index = ((x + y * (self.width as i32)) * 4) as usize;
        if index + 2 >= self.data.len() {
            return;
        }

        self.data[index] = c.r;
        self.data[index + 1] = c.g;
        self.data[index + 2] = c.b;
        self.data[index + 3] = c.a;
    }

    pub fn background(&mut self, c: Color) {
        for i in 0..(self.data.len() / 4) {
            self.data[i * 4] = c.r;
            self.data[i * 4 + 1] = c.g;
            self.data[i * 4 + 2] = c.b;
            self.data[i * 4 + 3] = c.a;
        }
    }

    pub fn flush(&self) {
        flush_canvas(&self.name, self.data.as_slice());
    }
}