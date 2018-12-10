use super::controller::SnakeController;
use graphics::types::Color;
use graphics::{Context, Graphics, Rectangle, Transformed};

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

    pub fn draw<G: Graphics>(&self, controller: &SnakeController, context: &Context, graphics: &mut G) {
        let ref settings = self.settings;
        let ref pos = controller.snake.pos;

        let transform = context.transform.trans(pos[0], pos[1]);
        Rectangle::new(settings.color).draw([0.0, 0.0, 10.0, 10.0], &context.draw_state, transform, graphics);
    }
}
