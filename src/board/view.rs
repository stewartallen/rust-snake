use graphics::character::CharacterCache;
use graphics::math::Matrix2d;
use graphics::types::{Color, FontSize, Vec2d};
use graphics::{Context, Graphics, Rectangle, Text, Transformed};

use super::controller::GameBoardController;

pub struct GameBoardViewSettings {
    position: Vec2d<f64>,

    background_color: Color,
    board_edge_color: Color,

    score_position: Vec2d<f64>,
    score_color: Color,
    score_font_size: FontSize,

    treat_color: Color,
}

impl GameBoardViewSettings {
    pub fn new() -> GameBoardViewSettings {
        GameBoardViewSettings {
            position: [10.0, 30.0],

            background_color: [0.3, 0.9, 0.5, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],

            score_position: [10.0, 20.0],
            score_color: [0.0, 0.0, 0.0, 1.0],
            score_font_size: 18,

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

    pub fn board_transform(&self, context: &Context) -> Matrix2d {
        let ref settings = self.settings;
        context.transform.trans(settings.position[0], settings.position[1])
    }

    pub fn draw<C, G>(&self, controller: &GameBoardController, glyphs: &mut C, context: &Context, graphics: &mut G)
    where
        C: CharacterCache,
        G: Graphics<Texture = <C as CharacterCache>::Texture>,
        <C as graphics::character::CharacterCache>::Error: std::fmt::Debug,
    {
        let ref settings = self.settings;
        let board_trans = self.board_transform(context);
        let board_rect = controller.game_board.board;

        // Draw board background.
        Rectangle::new(settings.background_color).draw(board_rect, &context.draw_state, board_trans, graphics);

        let score = format!("score: {}", controller.score());
        let score_trans = context
            .transform
            .trans(settings.score_position[0], settings.score_position[1]);

        Text::new_color(settings.score_color, settings.score_font_size)
            .draw(score.as_str(), glyphs, &context.draw_state, score_trans, graphics)
            .expect("Could not draw text");

        for treat in &controller.game_board.treats {
            Rectangle::new(settings.treat_color).draw(treat.shape, &context.draw_state, board_trans, graphics);
        }

        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, 1.0).draw(
            board_rect,
            &context.draw_state,
            board_trans,
            graphics,
        );
    }
}
