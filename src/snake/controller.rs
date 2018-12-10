use lazy_static::lazy_static;
use piston::input::GenericEvent;
use piston::input::{Button, Key, UpdateArgs};
use std::collections::HashMap;

use super::model::SnakeModel;

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

lazy_static! {
    static ref DIRECTIONS: HashMap<Direction, [i8; 2]> = {
        let mut map = HashMap::new();
        map.insert(Direction::NORTH, [0, -1]);
        map.insert(Direction::SOUTH, [0, 1]);
        map.insert(Direction::EAST, [1, 0]);
        map.insert(Direction::WEST, [-1, 0]);
        map
    };
}

pub struct SnakeController {
    pub snake: SnakeModel,
    direction: Direction,
}

impl SnakeController {
    pub fn new(snake: SnakeModel) -> SnakeController {
        SnakeController {
            snake,
            direction: Direction::EAST,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, event: &E) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => {
                    println!("UP");
                    self.direction = Direction::NORTH;
                }

                Key::Down => {
                    println!("DOWN");
                    self.direction = Direction::SOUTH;
                }

                Key::Left => {
                    println!("LEFT");
                    self.direction = Direction::WEST;
                }

                Key::Right => {
                    println!("RIGHT");
                    self.direction = Direction::EAST;
                }

                _ => {}
            }
        }
    }

    pub fn update(&mut self, _args: UpdateArgs) {
        let direction = DIRECTIONS.get(&self.direction).unwrap();
        self.snake.pos[0] += (direction[0] as f64) * self.snake.speed;
        self.snake.pos[1] += (direction[1] as f64) * self.snake.speed;
    }
}
