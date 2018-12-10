use rand::prelude::*;

pub fn random_start(extents: [f64; 2], border: f64) -> [f64; 2] {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(border, extents[0] - border);
    let y = rng.gen_range(border, extents[1] - border);
    [x, y]
}
