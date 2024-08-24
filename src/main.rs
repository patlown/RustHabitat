mod entity;
mod plot;
mod simulation;
mod simulation_space;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use plot::Plot;
use simulation::SimulationState;
use simulation_space::SimulationSpace;
use entity::EntityType;

const PLOT_WIDTH: f32 = 300.0;
const PLOT_HEIGHT: f32 = 200.0;
const PADDING: f32 = 20.0;

struct GameState {
    simulation: SimulationState,
    simulation_space: SimulationSpace,
    window_size: Vec2,
    prey_plot: Plot,
}

impl GameState {
    fn new(grid_width: i32, grid_height: i32, cell_size: f32) -> GameResult<GameState> {
        let simulation = SimulationState::new(grid_width, grid_height, cell_size)?;
        let window_size = Vec2::new(
            simulation.grid_size.x + PLOT_WIDTH + PADDING,
            f32::max(simulation.grid_size.y, PLOT_HEIGHT)
        );
        let plot_position = Vec2::new(
            simulation.grid_size.x + PADDING,
            (window_size.y - PLOT_HEIGHT) / 2.0
        );
        let prey_plot = Plot::new(
            plot_position, 
            Vec2::new(PLOT_WIDTH, PLOT_HEIGHT),
            "Prey Population".to_string(),
            |entity| entity.entity_type() == EntityType::Prey
        );
        let simulation_space = SimulationSpace::new(simulation.grid_size, simulation.cell_size);

        Ok(GameState {
            simulation,
            simulation_space,
            window_size,
            prey_plot,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        self.simulation.update(dt);
        self.prey_plot.update(&self.simulation.entities);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.9, 0.9, 1.0, 1.0]));

        // Draw simulation space
        self.simulation_space.draw(ctx, &mut canvas, &self.simulation)?;

        // Draw plot
        self.prey_plot.draw(ctx, &mut canvas)?;

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let grid_width = 40;
    let grid_height = 30;
    let cell_size = 20.0;

    let state = GameState::new(grid_width, grid_height, cell_size)?;

    let cb = ggez::ContextBuilder::new("ecosystem_simulation", "YourName")
        .window_setup(ggez::conf::WindowSetup::default().title("Ecosystem Simulation"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            state.window_size.x,
            state.window_size.y,
        ));

    let (ctx, event_loop) = cb.build()?;
    event::run(ctx, event_loop, state)
}