use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::Vec2;

struct GridState {
    grid_size: (i32, i32),
    cell_size: f32,
}

impl GridState {
    fn new(grid_size: (i32, i32), cell_size: f32) -> GameResult<GridState> {
        let s = GridState { 
            grid_size, 
            cell_size,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for GridState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.9, 0.9, 1.0, 1.0]),  // Light blue-gray background
        );

        let grid_color = Color::new(0.7, 0.7, 0.8, 0.3);  // Soft, semi-transparent gray

        // Draw grid lines
        for x in 0..=self.grid_size.0 {
            let x_pos = x as f32 * self.cell_size;
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    Vec2::new(x_pos, 0.0),
                    Vec2::new(x_pos, self.grid_size.1 as f32 * self.cell_size),
                ],
                1.0,  // Thinner line
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

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let grid_size = (40, 30);  // 40x30 grid
    let cell_size = 20.0;  // 20 pixels per cell

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