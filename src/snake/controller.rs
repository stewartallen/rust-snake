use lazy_static::lazy_static;
use piston::input::GenericEvent;
use piston::input::{Button, Key, UpdateArgs};
use std::collections::HashMap;

use super::model::SnakeModel;
use crate::util;

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
                    /* Make sure the snake can't turn on it's self. */
                    if self.direction != Direction::SOUTH {
                        self.direction = Direction::NORTH;
                    }
                }

                Key::Down => {
                    if self.direction != Direction::NORTH {
                        self.direction = Direction::SOUTH;
                    }
                }

                Key::Left => {
                    if self.direction != Direction::EAST {
                        self.direction = Direction::WEST;
                    }
                }

                Key::Right => {
                    if self.direction != Direction::WEST {
                        self.direction = Direction::EAST;
                    }
                }

                _ => {}
            }
        }
    }

    fn in_bounds(&self) -> bool {
        let ref snake = self.snake;
        let board_extents = [
            snake.board_extents[0] - snake.size[0] * 2.0,
            snake.board_extents[1] - snake.size[1] * 2.0,
        ];

        /* Check that the snake is in bounds, ie is colliding with the board */
        util::collision(
            [[snake.pos[0], snake.pos[1]], [snake.size[0], snake.size[1]]],
            [[snake.size[0], snake.size[1]], board_extents],
        )
    }

    pub fn update(&mut self, _args: UpdateArgs) -> bool {
        let direction = DIRECTIONS.get(&self.direction).unwrap();
        self.snake.pos[0] += (direction[0] as f64) * self.snake.speed;
        self.snake.pos[1] += (direction[1] as f64) * self.snake.speed;

        self.in_bounds()
    }
}
