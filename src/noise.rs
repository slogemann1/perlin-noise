use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use rand::{ SeedableRng, Rng };
use rand::rngs::StdRng;

///Struct, welches Perlin Noise Werte zurückgibt
pub struct NoiseGen<T: GradVector>
where T::U: Clone {
    ///Die Liste von möglichen Gradientenvectoren
    ///wenn None ist, werden die Vectoren ganz zufällig ausgewählt 
    vec_list: Option<Vec<T::U>>,
    ///Die Liste von generierten Gradientenvectoren an jeder stelle
    ///Die Vectoren werden nur dann ausgerechen, wenn sie gebraucht werden
    field: HashMap<T, T::U>,
    ///Zufallszahlengenerator der auf einen eingegebenen Seed basiert
    random: StdRng
}

impl<T: GradVector> NoiseGen::<T>
where T::U: Clone + Debug {
    ///Gibt ein NoiseGen Struct zurück, der aus den vorgegebenen Seed und Gradientenvectorenliste besteht
    pub fn new_from_seed_list(seed: u64, vec_list: Option<&Vec<T::U>>) -> Self {
        let mut vec_list_temp = None;
        if let Some(val) = vec_list {
            vec_list_temp = Some(val.clone());
        }

        NoiseGen {
            vec_list: vec_list_temp,
            field: HashMap::new(),
            random: StdRng::seed_from_u64(seed)
        }
    }

    ///Gibt ein NoiseGen Struct zurück, der aus den vorgegebenen Seed besteht
    pub fn new_from_seed(seed: u64) -> Self {
        NoiseGen {
            vec_list: T::default_gradient_list(),
            field: HashMap::new(),
            random: StdRng::seed_from_u64(seed)
        }
    }

    ///Gibt ein NoiseGen Struct zurück
    pub fn new() -> Self {
        NoiseGen {
            vec_list: T::default_gradient_list(),
            field: HashMap::new(),
            random: StdRng::seed_from_u64(rand::random())
        }
    }

    ///Gibt den Gradientenvector an der Stelle p wieder
    fn get_vector(&mut self, punkt: &T) -> T::U {
        self.field.entry(punkt.clone()).or_insert({
            if let Some(vec_list) = &self.vec_list {
                let index = (self.random.gen::<f64>() * (vec_list.len() as f64)) as usize;
                vec_list[index].clone()
            }
            else {
                T::new_normalized(&mut self.random)
            }
        }).clone()
    }

    ///Gibt den nächsten Perlin Noise Wert zurück
    pub fn next(&mut self, punkt: T::U) -> f64 {
        let p_list = T::get_nearest(&punkt); //Liste von den nächsten Punkten

        let mut g_list: Vec<T::U> = Vec::new(); //Liste von den Gradientenvectoren
        for p in &p_list {
            g_list.push(self.get_vector(p));
        }

        let mut dist_list: Vec<T::U> = Vec::new(); //Liste von den Distanzvectoren von den Ecken zum Punkt
        for p in &p_list {
            dist_list.push(T::dist(&punkt, p));
        }

        let w_list: Vec<f64> = T::get_w_list(&dist_list, &g_list); //Implementation in den Dimensionen selber (Siehe z.B. f32)
        let diff_list: Vec<f64> = T::get_dim_diff(&punkt); //Liste von den Unterschieden der jeweiligen Dimensionen

        T::normalize(Self::interpolate(&diff_list, &w_list))
    }

    fn interpolate(diff_list: &Vec<f64>, w_list: &Vec<f64>) -> f64 {
        //Nach Perlin's "Improving Noise"
        let blend = |d: f64| 10.0*d*d*d - 15.0*d*d*d*d + 6.0*d*d*d*d*d; //Blending function: 10X^3 − 15X^4 + 6X^5

        T::interpolate(diff_list, w_list, blend)
    }
}

///Ein Trait, welcher definiert welche Möglichen Dimensionen der NoiseGen struct haben kann 
pub trait GradVector: Sized + Clone + Eq + Hash + Debug {
    ///Typ von den Gradienten
    type U;

    ///Gibt eine Gradientenvectorenliste für den Dimension wieder
    fn default_gradient_list() -> Option<Vec<Self::U>>
    where Self: Sized + Clone;

    ///Erzeugt einen normalizierten Gradientenvector
    fn new_normalized(random: &mut StdRng) -> Self::U;

    ///Gibt die nächsten Punkte an
    fn get_nearest(p: &Self::U) -> Vec<Self>;

    ///Gibt den Unterschied zwischen den Punkten wieder
    fn dist(p1: &Self::U, p2: &Self) -> Self::U;

    ///Gibt die liste der Tangentenwerte wieder
    fn get_w_list(dist_list: &Vec<Self::U>, g_list: &Vec<Self::U>) -> Vec<f64>;

    ///Gibt die Anzahl der Dimensionen wieder
    fn dim() -> u32;

    ///Die Interpolationsfunktion der Dimension
    fn interpolate<F: Fn(f64) -> f64>(diff_list: &Vec<f64>, w_list: &Vec<f64>, blend: F) -> f64;

    ///Gibt die unterschiedlichen Differenzwerte für den Punkt an (1 pro Dimension) 
    fn get_dim_diff(punkt: &Self::U) -> Vec<f64>;

    ///Gibt den Perlin Noise Wert als Zahl zwischen 0 und 1 (ungefähr)
    fn normalize(noise_wert: f64) -> f64;
}

impl GradVector for i32 {
    type U = f64;

    fn default_gradient_list() -> Option<Vec<Self::U>> {
        None
    }

    fn new_normalized(random: &mut StdRng) -> Self::U {
        (random.gen::<Self::U>() * 2.0) - 1.0
    }

    fn get_nearest(p: &Self::U) -> Vec<Self> {
        vec![*p as i32, (*p as i32) + 1]
    }

    fn dist(p1: &Self::U, p2: &Self) -> Self::U {
        *p1 - (*p2 as f64)
    }

    fn get_w_list(dist_list: &Vec<Self::U>, g_list: &Vec<Self::U>) -> Vec<f64> {
        vec![dist_list[0] * g_list[0], dist_list[1] * g_list[1]]
    }

    fn dim() -> u32 {
        1
    }

    fn interpolate<F: Fn(f64) -> f64>(diff_list: &Vec<f64>, w_list: &Vec<f64>, blend: F) -> f64 {
        let x_b = blend(diff_list[0]);
        (1.0 - x_b) * w_list[0] + x_b * w_list[1]
    }

    fn get_dim_diff(punkt: &Self::U) -> Vec<f64> {
        vec![punkt - ((*punkt as i32) as f64)]
    }

    fn normalize(noise_wert: f64) -> f64 {
        //ungefähr
        noise_wert + 0.5
    }
}

impl GradVector for (i32, i32) {
    type U = (f64, f64);

    fn default_gradient_list() -> Option<Vec<Self::U>> {
        Some(
            vec![
                //Nach Perlin's "Improving Noise" (Rictungen von Quadrat zu den Eckpunkten)
                (1.0, 1.0), 
                (1.0, -1.0),
                (-1.0, 1.0),
                (-1.0, -1.0)
            ]
        )
    }

    fn new_normalized(random: &mut StdRng) -> Self::U {
        let rand: (f64, f64) = random.gen::<Self::U>();
        let mag = (rand.0 * rand.0 + rand.1 * rand.1).sqrt();

        (rand.0 / mag, rand.1 / mag)
    }

    fn get_nearest(p: &Self::U) -> Vec<Self> {
        let (x, y) = *p;

        vec![
            (x as i32, y as i32),
            ((x as i32) + 1, y as i32),
            (x as i32, (y as i32) + 1),
            ((x as i32) + 1, (y as i32) + 1)
        ]
    }

    fn dist(p1: &Self::U, p2: &Self) -> Self::U {
        (p1.0 - (p2.0 as f64), p1.1 - (p2.1 as f64))
    }

    fn get_w_list(dist_list: &Vec<Self::U>, g_list: &Vec<Self::U>) -> Vec<f64> {
        vec![
            dist_list[0].0 * g_list[0].0 + dist_list[0].1 * g_list[0].1, 
            dist_list[1].0 * g_list[1].0 + dist_list[1].1 * g_list[1].1,
            dist_list[2].0 * g_list[2].0 + dist_list[2].1 * g_list[2].1,
            dist_list[3].0 * g_list[3].0 + dist_list[3].1 * g_list[3].1
        ]
    }

    fn dim() -> u32 {
        2
    }

    fn interpolate<F: Fn(f64) -> f64>(diff_list: &Vec<f64>, w_list: &Vec<f64>, blend: F) -> f64 {
        let x_b = blend(diff_list[0]);
        let y_b = blend(diff_list[1]);

        let w1 = (1.0 - x_b) * w_list[0] + x_b * w_list[1];
        let w2 = (1.0 - x_b) * w_list[2] + x_b * w_list[3];

        (1.0 - y_b) * w1 + y_b * w2 
    }

    fn get_dim_diff(punkt: &Self::U) -> Vec<f64> {
        vec![
            punkt.0 - ((punkt.0 as i32) as f64),
            punkt.1 - ((punkt.1 as i32) as f64),
        ]
    }

    fn normalize(noise_wert: f64) -> f64 {
        (noise_wert + 1.0) / 2.0
    }
}