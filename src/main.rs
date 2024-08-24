mod entity;

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use entity::Entity;

struct GridState {
    grid_size: (i32, i32),
    cell_size: f32,
    entity: Entity,
}

impl GridState {
    fn new(grid_size: (i32, i32), cell_size: f32) -> GameResult<GridState> {
        let grid_width = grid_size.0 as f32 * cell_size;
        let grid_height = grid_size.1 as f32 * cell_size;
        
        let entity = Entity::new(
            grid_width / 2.0,
            grid_height / 2.0,
            cell_size * 0.8,
            Color::GREEN,
            grid_width,
            grid_height,
        );

        let s = GridState { 
            grid_size, 
            cell_size,
            entity,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for GridState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        self.entity.update(dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.9, 0.9, 1.0, 1.0]),
        );

        let grid_color = Color::new(0.7, 0.7, 0.8, 0.3);

        // Draw grid lines (same as before)
        for x in 0..=self.grid_size.0 {
            let x_pos = x as f32 * self.cell_size;
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    Vec2::new(x_pos, 0.0),
                    Vec2::new(x_pos, self.grid_size.1 as f32 * self.cell_size),
                ],
                1.0,
                grid_color,
            )?;
            canvas.draw(&line, Vec2::new(0.0, 0.0));
        }

        for y in 0..=self.grid_size.1 {
            let y_pos = y as f32 * self.cell_size;
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    Vec2::new(0.0, y_pos),
                    Vec2::new(self.grid_size.0 as f32 * self.cell_size, y_pos),
                ],
                1.0,
                grid_color,
            )?;
            canvas.draw(&line, Vec2::new(0.0, 0.0));
        }

        // Draw the entity
        self.entity.draw(ctx, &mut canvas)?;

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let grid_size = (40, 30);
    let cell_size = 20.0;

    let cb = ggez::ContextBuilder::new("grid_simulation", "YourName")
        .window_setup(ggez::conf::WindowSetup::default().title("Grid Simulation"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            grid_size.0 as f32 * cell_size,
            grid_size.1 as f32 * cell_size,
        ));

    let (ctx, event_loop) = cb.build()?;
    let state = GridState::new(grid_size, cell_size)?;
    event::run(ctx, event_loop, state)
}