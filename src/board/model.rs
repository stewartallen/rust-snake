use crate::util;
use graphics::types::Rectangle;

pub struct Treat {
    pub shape: Rectangle<f64>,
}

pub struct GameBoardModel {
    pub board: Rectangle<f64>,
    pub treats: Vec<Treat>,
    pub score: u32,
}

impl GameBoardModel {
    pub fn new(board: Rectangle) -> GameBoardModel {
        GameBoardModel {
            board,
            // Create with at least one treat
            treats: vec![Treat::new(board)],
            score: 0,
        }
    }
}

impl Treat {
    pub fn new(extents: Rectangle) -> Treat {
        let start = util::random_start(extents, 50.0);
        Treat {
            shape: [start[0], start[1], 10.0, 10.0],
        }
    }
}
