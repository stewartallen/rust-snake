use crate::util;

pub struct SnakeModel {
    pub board_extents: [f64; 2],
    pub pos: [f64; 2],
    pub speed: f64,
    pub size: [f64; 2],
}

impl SnakeModel {
    pub fn new(board_extents: [f64; 2], starting_speed: f64) -> SnakeModel {
        SnakeModel {
            pos: util::random_start(board_extents, 100.0),
            speed: starting_speed,
            board_extents,
            size: [10.0; 2],
        }
    }
}
