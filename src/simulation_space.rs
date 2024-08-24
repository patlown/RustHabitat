use ggez::graphics::{Canvas, Color, Mesh};
use ggez::glam::Vec2;
use ggez::{Context, GameResult};

use crate::simulation::SimulationState;
use crate::entity::Entity;

pub struct SimulationSpace {
    grid_size: Vec2,
    cell_size: f32,
}

impl SimulationSpace {
    pub fn new(grid_size: Vec2, cell_size: f32) -> Self {
        SimulationSpace {
            grid_size,
            cell_size,
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, simulation: &SimulationState) -> GameResult {
        self.draw_grid(ctx, canvas)?;
        self.draw_entities(ctx, canvas, &simulation.entities)?;
        Ok(())
    }

    fn draw_grid(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let grid_color = Color::new(0.7, 0.7, 0.8, 0.3);

        for x in (0..=self.grid_size.x as i32).step_by(self.cell_size as usize) {
            let x = x as f32;
            let line = Mesh::new_line(
                ctx,
                &[Vec2::new(x, 0.0), Vec2::new(x, self.grid_size.y)],
                1.0,
                grid_color,
            )?;
            canvas.draw(&line, Vec2::new(0.0, 0.0));
        }

        for y in (0..=self.grid_size.y as i32).step_by(self.cell_size as usize) {
            let y = y as f32;
            let line = Mesh::new_line(
                ctx,
                &[Vec2::new(0.0, y), Vec2::new(self.grid_size.x, y)],
                1.0,
                grid_color,
            )?;
            canvas.draw(&line, Vec2::new(0.0, 0.0));
        }
        Ok(())
    }

    fn draw_entities(&self, ctx: &mut Context, canvas: &mut Canvas, entities: &[Entity]) -> GameResult {
        for entity in entities {
            entity.draw(ctx, canvas)?;
        }
        Ok(())
    }
}