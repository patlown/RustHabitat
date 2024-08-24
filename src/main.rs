mod entity;
mod simulation;
mod simulation_space;
mod area_chart;

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use simulation::SimulationState;
use simulation_space::SimulationSpace;
use area_chart::{AreaChart, TrackingStat};
use entity::EntityType;

// Constants for chart dimensions
const CHART_WIDTH: f32 = 300.0;
const CHART_HEIGHT: f32 = 200.0;
const CHART_MARGIN: f32 = 20.0;

struct GameState {
    simulation: SimulationState,
    simulation_space: SimulationSpace,
    window_size: Vec2,
    predator_chart: AreaChart,
    prey_chart: AreaChart,
    plant_chart: AreaChart,
}

impl GameState {
    fn new(grid_width: i32, grid_height: i32, cell_size: f32) -> GameResult<GameState> {
        let simulation = SimulationState::new(grid_width, grid_height, cell_size)?;

        let predator_chart = AreaChart::new(
            Vec2::new(CHART_MARGIN, CHART_MARGIN),
            Vec2::new(CHART_WIDTH, CHART_HEIGHT),
            Color::RED,
            "Predators".to_string(),
            EntityType::Predator,
            TrackingStat::Population,
        );

        let prey_chart = AreaChart::new(
            Vec2::new(CHART_MARGIN, CHART_HEIGHT + CHART_MARGIN * 2.0),
            Vec2::new(CHART_WIDTH, CHART_HEIGHT),
            Color::BLUE,
            "Prey".to_string(),
            EntityType::Prey,
            TrackingStat::Population,
        );

        let plant_chart = AreaChart::new(
            Vec2::new(CHART_MARGIN, CHART_HEIGHT * 2.0 + CHART_MARGIN * 3.0),
            Vec2::new(CHART_WIDTH, CHART_HEIGHT),
            Color::GREEN,
            "Plants".to_string(),
            EntityType::Plant,
            TrackingStat::Population,
        );

        let window_size = Vec2::new(
            simulation.grid_size.x + CHART_WIDTH + CHART_MARGIN,
            simulation.grid_size.y + CHART_HEIGHT * 3.0 + CHART_MARGIN * 4.0,
        );

        let simulation_space = SimulationSpace::new(simulation.grid_size, simulation.cell_size);

        Ok(GameState {
            simulation,
            simulation_space,
            window_size,
            predator_chart,
            prey_chart,
            plant_chart,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        self.simulation.update(dt);

        // Update charts
        self.predator_chart.update(self.simulation.time, &self.simulation.entities);
        self.prey_chart.update(self.simulation.time, &self.simulation.entities);
        self.plant_chart.update(self.simulation.time, &self.simulation.entities);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Draw simulation space
        self.simulation_space.draw(ctx, &mut canvas, &self.simulation)?;

        // Draw area charts
        self.predator_chart.draw(ctx, &mut canvas)?;
        self.prey_chart.draw(ctx, &mut canvas)?;
        self.plant_chart.draw(ctx, &mut canvas)?;

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let grid_width = 50;
    let grid_height = 50;
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