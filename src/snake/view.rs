use super::controller::SnakeController;
use graphics::types::Color;
use graphics::{Context, Graphics, Rectangle, Transformed};
use rand::prelude::*;

pub struct SnakeViewSettings {
    pub color: Color,
}

impl SnakeViewSettings {
    pub fn new() -> SnakeViewSettings {
        SnakeViewSettings {
            color: [0.9, 0.6, 0.1, 1.0],
        }
    }
}

pub struct SnakeView {
    pub settings: SnakeViewSettings,
}

pub fn random_start(extents: [f64; 2]) -> [f64; 2] {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(100.0, extents[0] - 100.0);
    let y = rng.gen_range(100.0, extents[1] - 100.0);
    [x, y]
}

impl SnakeView {
    pub fn new(settings: SnakeViewSettings) -> SnakeView {
        SnakeView { settings }
    }

    pub fn draw<G: Graphics>(
        &self,
        controller: &SnakeController,
        context: &Context,
        graphics: &mut G,
    ) {
        let ref settings = self.settings;
        let ref pos = controller.snake.pos;

        let transform = context.transform.trans(pos[0], pos[1]);
        Rectangle::new(settings.color).draw(
            [0.0, 0.0, 10.0, 10.0],
            &context.draw_state,
            transform,
            graphics,
        );
    }
}
