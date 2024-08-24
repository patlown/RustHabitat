mod entity;
mod simulation;
mod simulation_space;
mod area_chart;

use ggez::{event, Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{EventHandler};
use ggez::glam::Vec2;
use simulation::SimulationState;
use simulation_space::SimulationSpace;
use area_chart::{AreaChart, TrackingStat};
use entity::EntityType;

// Constants for chart dimensions
const CHART_WIDTH: f32 = 300.0;
const CHART_HEIGHT: f32 = 200.0;
const CHART_MARGIN: f32 = 20.0;

// Updated constants for grid
const GRID_WIDTH: i32 = 40;
const GRID_HEIGHT: i32 = 50;
const CELL_SIZE: f32 = 15.0;

struct MainState {
    simulation: SimulationState,
    simulation_space: SimulationSpace,
    predator_chart: AreaChart,
    prey_chart: AreaChart,
    plant_chart: AreaChart,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let simulation = SimulationState::new(GRID_WIDTH, GRID_HEIGHT, CELL_SIZE)?;

        let simulation_space = SimulationSpace::new(Vec2::new(GRID_WIDTH as f32 * CELL_SIZE, GRID_HEIGHT as f32 * CELL_SIZE), CELL_SIZE);

        let chart_start_x = simulation_space.size.x + CHART_MARGIN;

        let predator_chart = AreaChart::new(
            Vec2::new(chart_start_x, CHART_MARGIN),
            Vec2::new(CHART_WIDTH, CHART_HEIGHT),
            Color::RED,
            "Predators".to_string(),
            EntityType::Predator,
            TrackingStat::Population,
        );

        let prey_chart = AreaChart::new(
            Vec2::new(chart_start_x, CHART_HEIGHT + CHART_MARGIN * 2.0),
            Vec2::new(CHART_WIDTH, CHART_HEIGHT),
            Color::BLUE,
            "Prey".to_string(),
            EntityType::Prey,
            TrackingStat::Population,
        );

        let plant_chart = AreaChart::new(
            Vec2::new(chart_start_x, CHART_HEIGHT * 2.0 + CHART_MARGIN * 3.0),
            Vec2::new(CHART_WIDTH, CHART_HEIGHT),
            Color::GREEN,
            "Plants".to_string(),
            EntityType::Plant,
            TrackingStat::Population,
        );

        Ok(MainState {
            simulation,
            simulation_space,
            predator_chart,
            prey_chart,
            plant_chart,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        self.simulation.update(dt);

        self.predator_chart.update(self.simulation.time, &self.simulation.entities);
        self.prey_chart.update(self.simulation.time, &self.simulation.entities);
        self.plant_chart.update(self.simulation.time, &self.simulation.entities);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        self.simulation_space.draw(ctx, &mut canvas, &self.simulation)?;

        self.predator_chart.draw(ctx, &mut canvas)?;
        self.prey_chart.draw(ctx, &mut canvas)?;
        self.plant_chart.draw(ctx, &mut canvas)?;

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let window_width = GRID_WIDTH as f32 * CELL_SIZE + CHART_WIDTH + CHART_MARGIN * 3.0;
    let window_height = (GRID_HEIGHT as f32 * CELL_SIZE).max(CHART_HEIGHT * 3.0 + CHART_MARGIN * 4.0);

    let cb = ggez::ContextBuilder::new("ecosystem_simulation", "Your Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Ecosystem Simulation"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(window_width, window_height));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}