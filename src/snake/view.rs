use super::controller::SnakeController;
use graphics::math::Matrix2d;
use graphics::types::Color;
use graphics::{Context, Graphics, Rectangle};

pub struct SnakeViewSettings {
    color: Color,
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

impl SnakeView {
    pub fn new(settings: SnakeViewSettings) -> SnakeView {
        SnakeView { settings }
    }

    pub fn draw<G: Graphics>(
        &self,
        controller: &SnakeController,
        transform: Matrix2d,
        context: &Context,
        graphics: &mut G,
    ) {
        let ref settings = self.settings;

        Rectangle::new(settings.color).draw(controller.snake.shape, &context.draw_state, transform, graphics);
    }
}
