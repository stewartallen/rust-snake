extern crate graphics;
extern crate piston;
extern crate piston_window;

use crate::board::controller::{GameBoardController, GameBoardControllerSettings};
use crate::board::model::GameBoardModel;
use crate::board::view::{GameBoardView, GameBoardViewSettings};
use crate::snake::controller::SnakeController;
use crate::snake::model::SnakeModel;
use crate::snake::view::{SnakeView, SnakeViewSettings};
use piston_window::*;

mod board;
mod snake;
mod util;

pub static WINDOW_SIZE: [f64; 2] = [800.0, 600.0];
pub static SNAKE_SPEED: f64 = 1.0;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rust Snake", WINDOW_SIZE)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let board = GameBoardModel::new(WINDOW_SIZE);
    let board_controller_settings = GameBoardControllerSettings::new();
    let mut board_controller = GameBoardController::new(board_controller_settings, board);
    let board_view_settings = GameBoardViewSettings::new();
    let board_view = GameBoardView::new(board_view_settings);

    let snake = SnakeModel::new(board_controller.game_board.size, SNAKE_SPEED);
    let mut snake_controller = SnakeController::new(snake);
    let snake_view_settings = SnakeViewSettings::new();
    let snake_view = SnakeView::new(snake_view_settings);

    let factory = window.factory.clone();
    let mut glyphs =
        Glyphs::new("assets/FiraSans-Regular.ttf", factory, TextureSettings::new()).expect("Could not load font");

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        snake_controller.event(&event);

        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            board_view.draw(&board_controller, &mut glyphs, &context, graphics);
            snake_view.draw(&snake_controller, &context, graphics);
        });

        if let Some(update) = event.update_args() {
            board_controller.update(update);
            snake_controller.update(update);
        }
    }
}
