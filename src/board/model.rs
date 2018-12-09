pub struct Treat {
    x: u32,
    y: u32,
}

pub struct GameBoardModel {
    pub treats: Vec<Treat>,
    pub score: u32,
}

impl GameBoardModel {
    pub fn new() -> GameBoardModel {
        GameBoardModel {
            treats: Vec::new(),
            score: 0,
        }
    }
}
