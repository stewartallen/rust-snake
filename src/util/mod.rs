use rand::prelude::*;

pub fn random_start(extents: [f64; 2], border: f64) -> [f64; 2] {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(border, extents[0] - border);
    let y = rng.gen_range(border, extents[1] - border);
    [x, y]
}

pub fn collision(rect1: [[f64; 2]; 2], rect2: [[f64; 2]; 2]) -> bool {
    rect1[0][0] < rect2[0][0] + rect2[1][0]
        && rect1[0][0] + rect1[1][0] > rect2[0][0]
        && rect1[0][1] < rect2[0][1] + rect2[1][1]
        && rect1[0][1] + rect1[1][1] > rect2[0][1]
}
