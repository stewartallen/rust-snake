use super::model::{GameBoardModel, Treat};
use crate::snake::controller::SnakeController;
use crate::util;
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

    pub fn update(&mut self, snake_controller: &SnakeController, _args: UpdateArgs) {
        let snake_shape = snake_controller.snake.shape;
        let mut score_inc = 0;

        self.game_board.treats.retain(|treat| {
            if util::collision(snake_shape, treat.shape) {
                score_inc += 1;
                return false;
            }
            true
        });

        self.game_board.score += score_inc;

        if self.game_board.treats.len() < self.settings.max_treats {
            if self.rng.gen_bool(self.settings.treat_rate) {
                self.game_board.treats.push(Treat::new(self.game_board.board));
            }
        }
    }

    pub fn score(&self) -> u32 {
        self.game_board.score
    }
}
