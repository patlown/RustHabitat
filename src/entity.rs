use ggez::graphics::{self, Color, Mesh};
use ggez::glam::Vec2;
use ggez::{Context, GameResult};

pub struct Entity {
    position: Vec2,
    size: f32,
    color: Color,
}

impl Entity {
    pub fn new(x: f32, y: f32, size: f32, color: Color) -> Self {
        Entity {
            position: Vec2::new(x, y),
            size,
            color,
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let circle = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.position,
            self.size / 2.0,
            0.1,
            self.color,
        )?;

        canvas.draw(&circle, graphics::DrawParam::default());
        Ok(())
    }

    pub fn update(&mut self, _dt: f32) {
    }
}