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

const ORIGINAL_SEED: u64 = 120398471023;

//Hier werden die Beispiele initializiert
lazy_static! {
    static ref NOISE_2D: Mutex<NoiseGen<(i32, i32)>> = {
        Mutex::new(NoiseGen::new_from_seed(get_seed()))
    };

    static ref DISPLAY_RAND: Mutex<Canvas> = {
        let canvas = Canvas::new(256, 256, "rand");
        canvas.set_title("Zufällige Zuordnung");
        canvas.set_pos(50, 75);
        canvas.set_h2_pos(22, 0);
        Mutex::new(canvas)
    };

    static ref DISPLAY_1D: Mutex<Canvas> = {
        let canvas = Canvas::new(256, 256, "1d");
        canvas.set_title("1D Perlin Noise");
        canvas.set_pos(400, 75);
        canvas.set_h2_pos(50, 0);
        Mutex::new(canvas)
    };

    static ref DISPLAY_T_WERT: Mutex<Canvas> = {
        let canvas = Canvas::new(256, 256, "1d_t");
        canvas.set_title("Tangentenwerte t1 und t2 im Interval [1, 2]");
        canvas.set_pos(595, 450);
        canvas.set_h2_pos(-85, 0);
        Mutex::new(canvas)
    };

    static ref DISPLAY_2D: Mutex<Canvas> = {
        let canvas = Canvas::new(256, 256, "2d");
        canvas.set_title("2D Perlin Noise (Animation)");
        Mutex::new(canvas)
    };

    static ref DISPLAY_2D_B: Mutex<Canvas> = {
        let canvas = Canvas::new(256, 256, "2d_b");
        canvas.set_title("2D Perlin Noise (Bild)");
        canvas.set_pos(800, 75);
        canvas.set_h2_pos(17, 0);
        Mutex::new(canvas)
    };

    static ref DISPLAY_2D_K: Mutex<Canvas> = {
        let canvas = Canvas::new(256, 256, "2d_k");
        canvas.set_title("2D Perlin Noise (Kreis)");
        canvas.set_pos(1200, 75);
        canvas.set_h2_pos(8, 0);
        Mutex::new(canvas)
    };

    static ref TIME: Mutex<f64> = {
        Mutex::new(0.0)
    };

    static ref SEED: Mutex<u64> = {
        Mutex::new(120398471023)
    };
}

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();

    //Alle Beispiele Zeichnen
    reset_canvas()
}

#[wasm_bindgen]
pub fn reset_canvas() {
    draw_zufaellig(); //Zufällige Verteilung von Werten
    draw_1d(); //Graf von 1-Dimensionaler Perlin Noise
    draw_tangentenwerte(); //Tangentenwerte t1 und t2
    draw_2d_bild(); //2-Dimensionaler Perlin Noise als Bild
    draw_2d_kreis(); //2-Dimensionaler Perlin Noise kreisförmiges Objekt
}

fn draw_zufaellig() {
    let seed = get_seed();
    let mut display_rand = match DISPLAY_RAND.lock() {
        Ok(val) => val,
        _ => return
    };

    let mut random = StdRng::seed_from_u64(seed);
    display_rand.background(Color::new(120, 120, 120));

    for x in 0..256 {
        let y = (random.gen::<f32>() * 256.0) as i32;

        display_rand.pixel(x, y, canvas::BLACK)
    }
    display_rand.flush();
}

fn draw_1d() {
    let seed = get_seed();
    let mut display_1d = match DISPLAY_1D.lock() {
        Ok(val) => val,
        _ => return
    };

    let mut noise: NoiseGen<i32> = NoiseGen::new_from_seed(seed);
    display_1d.background(Color::new(120, 120, 120));

    for x in 0..256 {
        let y = (noise.next((x as f64) / 128.0) * 256.0) as i32;

        display_1d.pixel(x, y, canvas::BLACK)
    }
    display_1d.flush();
}

fn draw_2d_bild() {
    let seed = get_seed();
    let mut display_2d_b = match DISPLAY_2D_B.lock() {
        Ok(val) => val,
        _ => return
    };

    let mut noise: NoiseGen<(i32, i32)> = NoiseGen::new_from_seed(seed);
    display_2d_b.background(Color::new(120, 120, 120));

    for x in 0..256 {
        for y in 0..256 {
            let input = ((x as f64) / 32.0, (y as f64) / 32.0);
            let c = ((noise.next(input)) * 256.0) as u8;

            display_2d_b.pixel(x, y, Color::new(c, c, c));
        }
    }
    display_2d_b.flush();
}

fn draw_2d_kreis() {
    let seed = get_seed();
    let mut display_2d_k = match DISPLAY_2D_K.lock() {
        Ok(val) => val,
        _ => return
    };

    let mut noise: NoiseGen<(i32, i32)> = NoiseGen::new_from_seed(seed);
    display_2d_k.background(Color::new(120, 120, 120));

    for i in 0..1440 {
        let input = (
            ((i as f64) / 4.0).to_radians().cos() + 1.0,
            ((i as f64) / 4.0).to_radians().sin() + 1.0
        );
        let r = (noise.next(input)) * 128.0;

        let x = ((i as f64) / 4.0).to_radians().cos() * r + 100.0;
        let y = ((i as f64) / 4.0).to_radians().sin() * r + 100.0;

        display_2d_k.pixel(x as i32, y as i32, canvas::BLACK);
    }
    display_2d_k.flush();
}

fn draw_tangentenwerte() {
    let seed = get_seed();
    let mut display_t_wert = match DISPLAY_T_WERT.lock() {
        Ok(val) => val,
        _ => return
    };
    console_log(&seed.to_string());
    let mut rand = StdRng::seed_from_u64(seed);

    let p1 = 1;
    let g1 = rand.gen::<f32>() * 2.0 - 1.0; //Zufälliger gradient zwischen -1 und 1
    let p2 = 2;
    let g2 = rand.gen::<f32>() * 2.0 - 1.0;

    display_t_wert.background(Color::new(120, 120, 120));

    for x in 0..256 {
        let x_wert = (x as f32) / 256.0 + (p1 as f32);

        //Tangentenwert t1
        let t1 = g1 * (x_wert - (p1 as f32));
        let t1 = t1 * 128.0 + 128.0; //Skalierung y von -1 bis 1 und x von p1 bis p2
        let t1 = 256.0 - t1; //In y Richtung spiegeln, weil Koordintensystem von links oben ausgeht
        
        //Tangentenwert t2
        let t2 = g2 * (x_wert - (p2 as f32));
        let t2 = t2 * 128.0 + 128.0;
        let t2 = 256.0 - t2;

        display_t_wert.pixel(x as i32, t1 as i32, canvas::BLACK); //t1 in schwarz
        display_t_wert.pixel(x as i32, t2 as i32, Color::new(255, 0, 0)); //t2 in rot
    }
    display_t_wert.flush();
}

fn get_seed() -> u64 {
    match SEED.lock() {
        Ok(val) => *val,
        _ => ORIGINAL_SEED
    }
}

#[wasm_bindgen]
pub fn set_seed(new_seed: u64) {
    match SEED.lock() {
        Ok(mut val) => { *val = new_seed; },
        _ => ()
    }
}