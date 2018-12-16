use crate::util;
use graphics::types::Rectangle;

pub struct SnakeModel {
    pub extents: Rectangle,
    pub shape: Rectangle,
    pub speed: f64,
}

impl SnakeModel {
    pub fn new(board_extents: Rectangle, starting_speed: f64) -> SnakeModel {
        let size = 10.0;
        let start = util::random_start(board_extents, 100.0);
        let extents: Rectangle<f64> = [
            board_extents[0] + size,
            board_extents[1] + size,
            board_extents[2] - size * 2.0,
            board_extents[3] - size * 2.0,
        ];

        SnakeModel {
            shape: [start[0], start[1], size, size],
            speed: starting_speed,
            extents,
        }
    }
}
