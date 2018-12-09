use super::model::GameBoardModel;
use piston::input::UpdateArgs;

pub struct GameBoardController {
    game_board: GameBoardModel,
}

impl GameBoardController {
    pub fn new(game_board: GameBoardModel) -> GameBoardController {
        GameBoardController { game_board }
    }

    pub fn update(&mut self, _args: UpdateArgs) {}

    pub fn score(&self) -> u32 {
        self.game_board.score
    }
}
