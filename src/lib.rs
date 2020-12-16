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
extern crate rand;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use rand::{ SeedableRng, Rng };
use rand::rngs::StdRng;

mod canvas;
mod noise;

use canvas::{ Canvas, Color };
use noise::NoiseGen;

const SEED: u64 = 120398471023;

lazy_static! {
    static ref NOISE_2D: Mutex<NoiseGen<(i32, i32)>> = {
        Mutex::new(NoiseGen::new_from_seed(SEED))
    };

    static ref DISPLAY_2D: Mutex<Canvas> = {
        let mut display_2d = Canvas::new(256, 256, "2d");
        Mutex::new(display_2d)
    };

    static ref TIME: Mutex<f64> = {
        Mutex::new(0.0)
    };
}

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();

    //Zufällige Verteilung von Werte
    create_h2("Random Distribution");
    let mut random = StdRng::seed_from_u64(SEED);
    let mut display_rand = Canvas::new(256, 256, "random");
    display_rand.background(Color::new(120, 120, 120));

    for x in 0..256 {
        let y = (random.gen::<f32>() * 256.0) as i32;

        display_rand.pixel(x, y, canvas::BLACK)
    }
    display_rand.flush();

    //1-Dimensionaler Perlin Noise
    create_h2("1D Perlin Noise");
    let mut noise: NoiseGen<i32> = NoiseGen::new_from_seed(SEED);
    let mut display_1d = Canvas::new(256, 256, "1d");
    display_1d.background(Color::new(120, 120, 120));

    for x in 0..256 {
        let y = (noise.next((x as f64) / 128.0) * 256.0) as i32;

        display_1d.pixel(x, y, canvas::BLACK)
    }
    display_1d.flush();

    //2-Dimensionaler Perlin Noise als Graph mit Animation
    create_h2("2D Perlin Noise (Animated)");
    lazy_static::initialize(&DISPLAY_2D);

    //2-Dimensionaler Perlin Noise
    create_h2("2D Perlin Noise (Stationary)");
    let mut noise: NoiseGen<(i32, i32)> = NoiseGen::new_from_seed(SEED);
    let mut display_2d = Canvas::new(256, 256, "2d_s");

    for x in 0..256 {
        for y in 0..256 {
            let input = ((x as f64) / 256.0, (y as f64) / 256.0);
            let c = (noise.next(input) * 256.0) as u8;

            display_2d.pixel(x, y, Color::new(c, c, c));
        }
    }
    display_2d.flush();
}

#[wasm_bindgen]
pub fn animateCallback() {
    let mut noise_2d = match NOISE_2D.lock() {
        Ok(val) => val,
        _ => return
    };
    let mut display_2d = match DISPLAY_2D.lock() {
        Ok(val) => val,
        _ => return
    };
    let mut t = match TIME.lock() {
        Ok(val) => val,
        _ => return
    };

    display_2d.background(Color::new(120, 120, 120));

    for x in 0..256 {
        let input = ((x as f64) / 128.0 + *t / 128.0, *t / 128.0);
        let y = (noise_2d.next(input) * 256.0) as i32;

        display_2d.pixel(x, y, canvas::BLACK)
    }
    display_2d.flush();

    console_log(&format!("{}", t));

    *t += 0.1;
}