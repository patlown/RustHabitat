use ggez::graphics::{self, Color, Mesh, DrawMode};
use ggez::glam::Vec2;
use ggez::{Context, GameResult};
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
pub enum EntityType {
    Predator,
    Prey,
    Plant,
}

pub struct Entity {
    entity_type: EntityType,
    position: Vec2,
    velocity: Vec2,
    size: f32,
    color: Color,
}

impl Entity {
    pub fn new_predator(x: f32, y: f32, size: f32) -> Self {
        Entity {
            entity_type: EntityType::Predator,
            position: Vec2::new(x, y),
            velocity: Vec2::new(0.0, 0.0),
            size,
            color: Color::RED,
        }
    }

    pub fn new_prey(x: f32, y: f32, size: f32) -> Self {
        Entity {
            entity_type: EntityType::Prey,
            position: Vec2::new(x, y),
            velocity: Vec2::new(0.0, 0.0),
            size,
            color: Color::BLUE,
        }
    }

    pub fn new_plant(x: f32, y: f32, size: f32) -> Self {
        Entity {
            entity_type: EntityType::Plant,
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
            size,
            color: Color::GREEN,
        }
    }

    pub fn update(&mut self, dt: f32, grid_size: Vec2) {
        match self.entity_type {
            EntityType::Predator | EntityType::Prey => {
                let mut rng = rand::thread_rng();
                self.velocity += Vec2::new(
                    rng.gen_range(-50.0..50.0) * dt,
                    rng.gen_range(-50.0..50.0) * dt,
                );

                let max_speed = if self.entity_type == EntityType::Predator { 120.0 } else { 100.0 };
                if self.velocity.length() > max_speed {
                    self.velocity = self.velocity.normalize() * max_speed;
                }

                self.position += self.velocity * dt;

                // Bounce off edges
                if self.position.x - self.size/2.0 < 0.0 || self.position.x + self.size/2.0 > grid_size.x {
                    self.velocity.x = -self.velocity.x;
                }
                if self.position.y - self.size/2.0 < 0.0 || self.position.y + self.size/2.0 > grid_size.y {
                    self.velocity.y = -self.velocity.y;
                }

                // Clamp position to grid
                self.position.x = self.position.x.clamp(self.size/2.0, grid_size.x - self.size/2.0);
                self.position.y = self.position.y.clamp(self.size/2.0, grid_size.y - self.size/2.0);
            }
            EntityType::Plant => {
                // Plants don't move
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            self.position,
            self.size / 2.0,
            0.1,
            self.color,
        )?;
        canvas.draw(&circle, graphics::DrawParam::default());

        if self.entity_type != EntityType::Plant {
            // Draw eyes for predators and prey
            let eye_size = self.size * 0.3;
            let eye_distance = self.size * 0.2;
            let left_eye_pos = self.position + Vec2::new(-eye_distance, -eye_distance);
            let right_eye_pos = self.position + Vec2::new(eye_distance, -eye_distance);

            let eye_white = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                Vec2::ZERO,
                eye_size / 2.0,
                0.1,
                Color::WHITE,
            )?;
            canvas.draw(&eye_white, graphics::DrawParam::new().dest(left_eye_pos));
            canvas.draw(&eye_white, graphics::DrawParam::new().dest(right_eye_pos));

            let pupil_size = eye_size * 0.6;
            let pupil = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                Vec2::ZERO,
                pupil_size / 2.0,
                0.1,
                Color::BLACK,
            )?;
            canvas.draw(&pupil, graphics::DrawParam::new().dest(left_eye_pos));
            canvas.draw(&pupil, graphics::DrawParam::new().dest(right_eye_pos));
        }

        Ok(())
    }
}