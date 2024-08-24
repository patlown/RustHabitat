use ggez::glam::Vec2;
use ggez::GameResult;
use rand::Rng;

use crate::entity::{Entity, EntityType};

pub struct SimulationState {
    pub grid_size: Vec2,
    pub cell_size: f32,
    pub entities: Vec<Entity>,
}

impl SimulationState {
    pub fn new(grid_width: i32, grid_height: i32, cell_size: f32) -> GameResult<Self> {
        let grid_size = Vec2::new(grid_width as f32 * cell_size, grid_height as f32 * cell_size);
        let mut entities = Vec::new();
        let mut rng = rand::thread_rng();

        // Create predators
        for _ in 0..3 {
            entities.push(Entity::new_predator(
                rng.gen_range(0.0..grid_size.x),
                rng.gen_range(0.0..grid_size.y),
                cell_size * 0.8,
            ));
        }

        // Create prey
        for _ in 0..10 {
            entities.push(Entity::new_prey(
                rng.gen_range(0.0..grid_size.x),
                rng.gen_range(0.0..grid_size.y),
                cell_size * 0.6,
            ));
        }

        // Create plants
        for _ in 0..20 {
            entities.push(Entity::new_plant(
                rng.gen_range(0.0..grid_size.x),
                rng.gen_range(0.0..grid_size.y),
                cell_size * 0.4,
            ));
        }

        Ok(SimulationState {
            grid_size,
            cell_size,
            entities,
        })
    }

    pub fn update(&mut self, dt: f32) {
        for entity in &mut self.entities {
            entity.update(dt, self.grid_size);
        }
    }
}