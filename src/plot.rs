use ggez::graphics::{self, Color, DrawMode, Mesh, Rect};
use ggez::glam::Vec2;
use ggez::{Context, GameResult};

pub struct Plot {
    position: Vec2,
    size: Vec2,
}

impl Plot {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Plot { position, size }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let plot_rect = Rect::new(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y
        );

        // Draw plot background
        let plot_background = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            plot_rect,
            Color::WHITE,
        )?;
        canvas.draw(&plot_background, Vec2::new(0.0, 0.0));

        // Draw plot border
        let plot_border = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            plot_rect,
            Color::BLACK,
        )?;
        canvas.draw(&plot_border, Vec2::new(0.0, 0.0));

        // Draw placeholder text
        let plot_text = graphics::Text::new("Population Plot\n(Placeholder)");
        let text_pos = Vec2::new(
            self.position.x + self.size.x / 2.0,
            self.position.y + self.size.y / 2.0
        );
        canvas.draw(
            &plot_text,
            graphics::DrawParam::default()
                .color(Color::BLACK)
                .dest(text_pos)
                .offset(Vec2::new(0.5, 0.5))
        );

        Ok(())
    }
}