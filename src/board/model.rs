use crate::util;

pub struct Treat {
    pub pos: [f64; 2],
}

pub struct GameBoardModel {
    pub size: [f64; 2],
    pub treats: Vec<Treat>,
    pub score: u32,
}

impl GameBoardModel {
    pub fn new(window_size: [f64; 2]) -> GameBoardModel {
        GameBoardModel {
            size: [window_size[0] - 20.0, window_size[1] - 20.0],
            // Create with at least one treat
            treats: vec![Treat::new(window_size)],
            score: 0,
        }
    }
}

impl Treat {
    pub fn new(extents: [f64; 2]) -> Treat {
        Treat {
            pos: util::random_start(extents, 50.0),
        }
    }
}
