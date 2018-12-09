use super::controller::GameBoardController;
use graphics::character::CharacterCache;
use graphics::types::{Color, FontSize};
use graphics::{Context, Graphics, Rectangle, Text, Transformed};

#[derive(Copy, Clone)]
pub struct GameBoardViewSettings {
    pub position: [f64; 2],
    pub size: [f64; 2],
    pub background_color: Color,
    pub board_edge_color: Color,
    pub score_color: Color,
    pub score_font_size: FontSize,
}

impl GameBoardViewSettings {
    pub fn new(window_size: (f64, f64)) -> GameBoardViewSettings {
        GameBoardViewSettings {
            position: [10.0; 2],
            size: [window_size.0 - 20.0, window_size.1 - 20.0],
            background_color: [0.3, 0.9, 0.5, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            score_color: [0.0, 0.0, 0.0, 1.0],
            score_font_size: 18,
        }
    }
}

pub struct GameBoardView {
    pub settings: GameBoardViewSettings,
}

impl GameBoardView {
    pub fn new(settings: GameBoardViewSettings) -> GameBoardView {
        GameBoardView { settings }
    }

    pub fn draw<C, G>(
        &self,
        controller: &GameBoardController,
        glyphs: &mut C,
        context: &Context,
        graphics: &mut G,
    ) where
        C: CharacterCache,
        G: Graphics<Texture = <C as CharacterCache>::Texture>,
        <C as graphics::character::CharacterCache>::Error: std::fmt::Debug,
    {
        let ref settings = self.settings;
        let board_rect = [
            settings.position[0],
            settings.position[1],
            settings.size[0],
            settings.size[1],
        ];

        // Draw board background.
        Rectangle::new(settings.background_color).draw(
            board_rect,
            &context.draw_state,
            context.transform,
            graphics,
        );

        let score = format!("score: {}", controller.score());
        let transform = context.transform.trans(settings.size[0] - 80.0, 30.0);
        Text::new_color(settings.score_color, settings.score_font_size)
            .draw(
                score.as_str(),
                glyphs,
                &context.draw_state,
                transform,
                graphics,
            )
            .expect("Could not draw text");

        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, 1.0).draw(
            board_rect,
            &context.draw_state,
            context.transform,
            graphics,
        );
    }
}
