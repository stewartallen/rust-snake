use crate::util;

pub struct Treat {
    pub pos: [f64; 2],
    pub size: [f64; 2],
}

pub struct GameBoardModel {
    pub size: [f64; 2],
    pub treats: Vec<Treat>,
    pub score: u32,
}

impl GameBoardModel {
    pub fn new(size: [f64; 2]) -> GameBoardModel {
        GameBoardModel {
            size,
            // Create with at least one treat
            treats: vec![Treat::new(size)],
            score: 0,
        }
    }
}

impl Treat {
    pub fn new(extents: [f64; 2]) -> Treat {
        Treat {
            pos: util::random_start(extents, 50.0),
            size: [10.0; 2],
        }
    }
}
