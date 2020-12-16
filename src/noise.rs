use std::collections::HashMap;
use std::hash::Hash;
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
where T::U: Clone {
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
    fn get_vector(&mut self, p: T) -> T::U {
        self.field.entry(p).or_insert({
            if let Some(vec_list) = &self.vec_list {
                let index = (self.random.gen::<f64>() * (vec_list.len() as f64)) as usize;
                vec_list[index].clone()
            }
            else {
                T::new_normalized(&mut self.random)
            }
        }).clone()
    }
}

impl NoiseGen<i32> {
    ///Gibt den nächsten Perlin Noise Wert zurück
    pub fn next(&mut self, p: f64) -> f64 {
        let p1 = p as i32; //floor(p)
        let p2 = p1 + 1; //ceil(p)

        let g2 = self.get_vector(p2);
        let g1 = self.get_vector(p1);

        let p1_diff = p - (p1 as f64);
        let p2_diff = p - (p2 as f64);

        let w1 = p1_diff * g1;
        let w2 = p2_diff * g2;

        Self::interpolate(p1_diff, w1, w2) + 0.5
    }

    fn interpolate(dx: f64, w1: f64, w2: f64) -> f64 {
        let blended = 10.0*dx*dx*dx - 15.0*dx*dx*dx*dx + 6.0*dx*dx*dx*dx*dx; //Blending function: 10X^3 − 15X^4 + 6X^5
        (1.0 - blended) * w1 + blended * w2
    }
}

impl NoiseGen<(i32, i32)> {
    ///Gibt den nächsten Perlin Noise Wert zurück
    pub fn next(&self, p: f64) -> f64 {
        0.0
    }
}

///Ein Trait, welcher definiert welche Möglichen Dimensionen der NoiseGen struct haben kann 
pub trait GradVector: Sized + Clone + Eq + Hash {
    ///Typ von den Gradienten
    type U;

    ///Gibt eine Gradientenvectorenliste für den Dimension wieder
    fn default_gradient_list() -> Option<Vec<Self::U>>
    where Self: Sized + Clone;

    ///Erzeugt einen normalizierten Gradientenvector
    fn new_normalized(random: &mut StdRng) -> Self::U;
}

impl GradVector for i32 {
    type U = f64;

    fn default_gradient_list() -> Option<Vec<Self::U>> {
        None
    }

    fn new_normalized(random: &mut StdRng) -> Self::U {
        (random.gen::<Self::U>() * 2.0) - 1.0
    }
}

impl GradVector for (i32, i32) {
    type U = (f64, f64);

    fn default_gradient_list() -> Option<Vec<Self::U>> {
        None
    }

    fn new_normalized(random: &mut StdRng) -> Self::U {
        (0.0, 0.0)
    }
}
