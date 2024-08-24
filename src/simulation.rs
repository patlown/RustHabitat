use ggez::glam::Vec2;
use ggez::GameResult;
use rand::Rng;

use crate::entity::Entity;

pub struct SimulationState {
    pub grid_size: Vec2,
    pub cell_size: f32,
    pub entities: Vec<Entity>,
    pub time: f32,
    rng: rand::rngs::ThreadRng,
    next_spawn_time: f32,
}

impl SimulationState {
    pub fn new(grid_width: i32, grid_height: i32, cell_size: f32) -> GameResult<Self> {
        let grid_size = Vec2::new(grid_width as f32 * cell_size, grid_height as f32 * cell_size);
        let mut entities = Vec::new();
        let mut rng = rand::thread_rng();

        // Create initial entities
        for _ in 0..1 {
            entities.push(Entity::new_predator(
                rng.gen_range(0.0..grid_size.x),
                rng.gen_range(0.0..grid_size.y),
                cell_size * 0.8,
            ));
        }

        for _ in 0..3 {
            entities.push(Entity::new_prey(
                rng.gen_range(0.0..grid_size.x),
                rng.gen_range(0.0..grid_size.y),
                cell_size * 0.6,
            ));
        }

        for _ in 0..5 {
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
            time: 0.0,
            rng,
            next_spawn_time: 2.0, // First entity will spawn after 2 seconds
        })
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;

        // Update existing entities
        for entity in &mut self.entities {
            entity.update(dt, self.grid_size);
        }

        // Spawn new entities over time
        if self.time >= self.next_spawn_time {
            self.spawn_new_entity();
            self.next_spawn_time = self.time + self.rng.gen_range(1.0..3.0); // Set next spawn time
        }
    }

    fn spawn_new_entity(&mut self) {
        let entity_type = self.rng.gen_range(0..3);
        let new_entity = match entity_type {
            0 => Entity::new_predator(
                self.rng.gen_range(0.0..self.grid_size.x),
                self.rng.gen_range(0.0..self.grid_size.y),
                self.cell_size * 0.8,
            ),
            1 => Entity::new_prey(
                self.rng.gen_range(0.0..self.grid_size.x),
                self.rng.gen_range(0.0..self.grid_size.y),
                self.cell_size * 0.6,
            ),
            _ => Entity::new_plant(
                self.rng.gen_range(0.0..self.grid_size.x),
                self.rng.gen_range(0.0..self.grid_size.y),
                self.cell_size * 0.4,
            ),
        };
        self.entities.push(new_entity);
    }
}