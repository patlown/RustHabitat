use ggez::graphics::{self, Canvas, Color};
use ggez::glam::Vec2;
use ggez::{Context, GameResult};

use crate::simulation::SimulationState;

pub struct SimulationSpace {
    pub size: Vec2,
    cell_size: f32,
}

impl SimulationSpace {
    pub fn new(grid_size: Vec2, cell_size: f32) -> Self {
        SimulationSpace {
            size: grid_size,
            cell_size,
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, simulation: &SimulationState) -> GameResult {
        // Draw light gray background for the grid
        let background = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, self.size.x, self.size.y),
            Color::from_rgb(220, 220, 220), // Light gray
        )?;
        canvas.draw(&background, Vec2::ZERO);

        // Draw grid lines
        for i in 0..=self.size.x as i32 / self.cell_size as i32 {
            let x = i as f32 * self.cell_size;
            let line = graphics::Mesh::new_line(
                ctx,
                &[Vec2::new(x, 0.0), Vec2::new(x, self.size.y)],
                1.0,
                Color::from_rgb(200, 200, 200), // Slightly darker gray for grid lines
            )?;
            canvas.draw(&line, Vec2::ZERO);
        }
        for i in 0..=self.size.y as i32 / self.cell_size as i32 {
            let y = i as f32 * self.cell_size;
            let line = graphics::Mesh::new_line(
                ctx,
                &[Vec2::new(0.0, y), Vec2::new(self.size.x, y)],
                1.0,
                Color::from_rgb(200, 200, 200), // Slightly darker gray for grid lines
            )?;
            canvas.draw(&line, Vec2::ZERO);
        }

        // Draw entities
        for entity in &simulation.entities {
            entity.draw(ctx, canvas)?;
        }

        Ok(())
    }
}