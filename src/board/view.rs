use super::controller::GameBoardController;
use graphics::character::CharacterCache;
use graphics::types::{Color, FontSize};
use graphics::{Context, Graphics, Rectangle, Text, Transformed};

pub struct GameBoardViewSettings {
    position: [f64; 2],

    background_color: Color,
    board_edge_color: Color,

    score_color: Color,
    score_font_size: FontSize,

    treat_size: [f64; 2],
    treat_color: Color,
}

impl GameBoardViewSettings {
    pub fn new() -> GameBoardViewSettings {
        GameBoardViewSettings {
            position: [10.0; 2],
            background_color: [0.3, 0.9, 0.5, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            score_color: [0.0, 0.0, 0.0, 1.0],
            score_font_size: 18,
            treat_size: [10.0, 10.0],
            treat_color: [1.0, 0.0, 0.0, 1.0],
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

    pub fn draw<C, G>(&self, controller: &GameBoardController, glyphs: &mut C, context: &Context, graphics: &mut G)
    where
        C: CharacterCache,
        G: Graphics<Texture = <C as CharacterCache>::Texture>,
        <C as graphics::character::CharacterCache>::Error: std::fmt::Debug,
    {
        let ref settings = self.settings;
        let ref board_size = controller.game_board.size;

        let board_rect = [settings.position[0], settings.position[1], board_size[0], board_size[1]];

        // Draw board background.
        Rectangle::new(settings.background_color).draw(board_rect, &context.draw_state, context.transform, graphics);

        let score = format!("score: {}", controller.score());
        let transform = context.transform.trans(board_size[0] - 80.0, 30.0);
        Text::new_color(settings.score_color, settings.score_font_size)
            .draw(score.as_str(), glyphs, &context.draw_state, transform, graphics)
            .expect("Could not draw text");

        for treat in &controller.game_board.treats {
            let transform = context.transform.trans(treat.pos[0], treat.pos[1]);
            Rectangle::new(settings.treat_color).draw(
                [0.0, 0.0, settings.treat_size[0], settings.treat_size[1]],
                &context.draw_state,
                transform,
                graphics,
            );
        }

        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, 1.0).draw(
            board_rect,
            &context.draw_state,
            context.transform,
            graphics,
        );
    }
}
