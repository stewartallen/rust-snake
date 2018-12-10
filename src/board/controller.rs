use super::model::{GameBoardModel, Treat};
use crate::snake::controller::SnakeController;
use crate::snake::model::SnakeModel;
use piston::input::UpdateArgs;
use rand::prelude::*;

pub struct GameBoardControllerSettings {
    max_treats: usize,
    treat_rate: f64,
}

pub struct GameBoardController {
    pub game_board: GameBoardModel,
    settings: GameBoardControllerSettings,
    rng: ThreadRng,
}

impl GameBoardControllerSettings {
    pub fn new() -> GameBoardControllerSettings {
        GameBoardControllerSettings {
            max_treats: 10,
            treat_rate: 0.002,
        }
    }
}

impl GameBoardController {
    pub fn new(settings: GameBoardControllerSettings, game_board: GameBoardModel) -> GameBoardController {
        GameBoardController {
            settings,
            game_board,
            rng: rand::thread_rng(),
        }
    }

    fn check_bounds(game_board: &GameBoardModel, snake: &SnakeModel) -> bool {
        let x = snake.pos[0];
        let y = snake.pos[1];
        let extents_x = game_board.size[0];
        let extents_y = game_board.size[1];

        /* TODO: Refactor this to use the real board extents. */
        x > 12.0 && x < (extents_x - 2.0) && y > 12.0 && y < (extents_y - 2.0)
    }

    pub fn update(&mut self, _args: UpdateArgs, snake_controller: &SnakeController) -> bool {
        let ref snake = snake_controller.snake;
        if !Self::check_bounds(&self.game_board, snake) {
            return false;
        }

        if self.game_board.treats.len() < self.settings.max_treats {
            if self.rng.gen_bool(self.settings.treat_rate) {
                self.game_board.treats.push(Treat::new(self.game_board.size));
            }
        }

        true
    }

    pub fn score(&self) -> u32 {
        self.game_board.score
    }
}
