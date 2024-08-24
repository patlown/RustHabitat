mod entity;

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use rand::Rng;
use entity::Entity;

struct GridState {
    grid_size: Vec2,
    cell_size: f32,
    entities: Vec<Entity>,
}

impl GridState {
    fn new(grid_width: i32, grid_height: i32, cell_size: f32) -> GameResult<GridState> {
        let grid_size = Vec2::new(grid_width as f32 * cell_size, grid_height as f32 * cell_size);
        let mut entities = Vec::new();
        let mut rng = rand::thread_rng();

        // Create predators
        for _ in 0..3 {
            entities.push(Entity::new_predator(
                rng.gen_range(0.0..grid_size.x),
                rng.gen_range(0.0..grid_size.y),
                cell_size,
            ));
        }

        // Create prey
        for _ in 0..10 {
            entities.push(Entity::new_prey(
                rng.gen_range(0.0..grid_size.x),
                rng.gen_range(0.0..grid_size.y),
                cell_size * 0.75,
            ));
        }

        // Create plants
        for _ in 0..20 {
            entities.push(Entity::new_plant(
                rng.gen_range(0.0..grid_size.x),
                rng.gen_range(0.0..grid_size.y),
                cell_size * 0.5,
            ));
        }

        Ok(GridState { 
            grid_size,
            cell_size,
            entities,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GridState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        
        // Update all entities
        for entity in &mut self.entities {
            entity.update(dt, self.grid_size);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.9, 0.9, 1.0, 1.0]));

        // Draw grid lines
        let grid_color = Color::new(0.7, 0.7, 0.8, 0.3);
        for x in (0..=self.grid_size.x as i32).step_by(self.cell_size as usize) {
            let x = x as f32;
            let line = graphics::Mesh::new_line(
                ctx,
                &[Vec2::new(x, 0.0), Vec2::new(x, self.grid_size.y)],
                1.0,
                grid_color,
            )?;
            canvas.draw(&line, Vec2::new(0.0, 0.0));
        }
        for y in (0..=self.grid_size.y as i32).step_by(self.cell_size as usize) {
            let y = y as f32;
            let line = graphics::Mesh::new_line(
                ctx,
                &[Vec2::new(0.0, y), Vec2::new(self.grid_size.x, y)],
                1.0,
                grid_color,
            )?;
            canvas.draw(&line, Vec2::new(0.0, 0.0));
        }

        // Draw all entities
        for entity in &self.entities {
            entity.draw(ctx, &mut canvas)?;
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let grid_width = 40;
    let grid_height = 30;
    let cell_size = 20.0;

    let cb = ggez::ContextBuilder::new("grid_simulation", "YourName")
        .window_setup(ggez::conf::WindowSetup::default().title("Ecosystem Simulation"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            grid_width as f32 * cell_size,
            grid_height as f32 * cell_size,
        ));

    let (ctx, event_loop) = cb.build()?;
    let state = GridState::new(grid_width, grid_height, cell_size)?;
    event::run(ctx, event_loop, state)
}