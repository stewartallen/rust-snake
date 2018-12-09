use super::model::SnakeModel;
use piston::input::GenericEvent;
use piston::input::{Button, Key, UpdateArgs};

pub struct SnakeController {
    pub snake: SnakeModel,
}

impl SnakeController {
    pub fn new(snake: SnakeModel) -> SnakeController {
        SnakeController { snake }
    }

    pub fn event<E: GenericEvent>(&mut self, event: &E) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => {
                    println!("UP");
                }

                Key::Down => {
                    println!("DOWN");
                }

                Key::Left => {
                    println!("LEFT");
                }

                Key::Right => {
                    println!("RIGHT");
                }

                _ => {}
            }
        }
    }

    pub fn update(&mut self, _args: UpdateArgs) {
        self.snake.pos[0] += self.snake.speed;
        self.snake.pos[1] += self.snake.speed;
    }
}
