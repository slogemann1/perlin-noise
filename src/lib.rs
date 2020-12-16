//Code für WebAssembly
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
mod utils;

//Verändern der Allocators für kleinerer Binärgröße
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//Funktionen von js importiert
#[wasm_bindgen(module = "/src/binding/my_exports.js")]
extern {
    #[wasm_bindgen]
    fn create_h2(s: &str);

    #[wasm_bindgen]
    fn console_log(s: &str);
}

//Mein Code beginnt hier
extern crate canvas_display;
extern crate embedded_graphics;
extern crate rand;

use canvas_display::prelude::*;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::rectangle::Rectangle,
    style::PrimitiveStyleBuilder
};
use rand::{ SeedableRng, Rng };
use rand::rngs::StdRng;

mod noise;
use noise::NoiseGen;

const SEED: u64 = 120398471023;

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();

    //Zufällige Verteilung von Werte
    create_h2("Random Distribution");
    let mut random = StdRng::seed_from_u64(SEED);
    let mut display_rand = new_canvas("random");

    for x in 0..256 {
        let y = (random.gen::<f32>() * 256.0) as i32;

        Pixel(Point::new(x, y), Rgb565::BLACK).draw(&mut display_rand).unwrap();
    }

    //1-Dimensionaler Perlinn Noise
    create_h2("1D Perlin Noise");
    let mut noise: NoiseGen<i32> = NoiseGen::new_from_seed(SEED);
    let mut display_1d = new_canvas("1d");

    for x in 0..256 {
        let y = (noise.next((x as f64) / 128.0) * 256.0) as i32;

        Pixel(Point::new(x, y), Rgb565::BLACK).draw(&mut display_1d).unwrap();
    }
}

///Funktion, die einen neuen Canvas mit dem gegebenen id auf der Website erstellt
fn new_canvas(id: &str) -> CanvasDisplay {
    let mut display = CanvasDisplay::new(256, 256, id).unwrap();
    let (r, g, b) = (120, 120, 120);
    let bg_color = PrimitiveStyleBuilder::new().fill_color(Rgb565::new((r * 32 / 256) as u8, (g * 64 / 256) as u8, (b * 32 / 256) as u8)).build();
    let background = Rectangle::new(Point::new(0, 0), Point::new(256, 256)).into_styled(bg_color);
    background.draw(&mut display).unwrap();

    display
}