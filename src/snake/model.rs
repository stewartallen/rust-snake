use crate::util;

pub struct SnakeModel {
    pub extents: [f64; 2],
    pub pos: [f64; 2],
    pub speed: f64,
}

impl SnakeModel {
    pub fn new(extents: [f64; 2], starting_speed: f64) -> SnakeModel {
        SnakeModel {
            pos: util::random_start(extents, 100.0),
            speed: starting_speed,
            extents,
        }
    }
}
