use ggez::graphics::{self, Color, Mesh};
use ggez::glam::Vec2;
use ggez::{Context, GameResult};
use rand::Rng;

pub struct Entity {
    position: Vec2,
    velocity: Vec2,
    size: f32,
    color: Color,
    grid_size: Vec2,
}

impl Entity {
    pub fn new(x: f32, y: f32, size: f32, color: Color, grid_width: f32, grid_height: f32) -> Self {
        Entity {
            position: Vec2::new(x, y),
            velocity: Vec2::new(0.0, 0.0),
            size,
            color,
            grid_size: Vec2::new(grid_width, grid_height),
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let circle = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.position,
            self.size / 2.0,
            0.1,
            self.color,
        )?;

        canvas.draw(&circle, graphics::DrawParam::default());
        Ok(())
    }

    pub fn update(&mut self, dt: f32) {
        // Random change in velocity
        let mut rng = rand::thread_rng();
        self.velocity += Vec2::new(
            rng.gen_range(-50.0..50.0) * dt,
            rng.gen_range(-50.0..50.0) * dt,
        );

        // Limit max speed
        const MAX_SPEED: f32 = 100.0;
        if self.velocity.length() > MAX_SPEED {
            self.velocity = self.velocity.normalize() * MAX_SPEED;
        }

        // Update position
        self.position += self.velocity * dt;

        // Bounce off edges
        if self.position.x - self.size/2.0 < 0.0 || self.position.x + self.size/2.0 > self.grid_size.x {
            self.velocity.x = -self.velocity.x;
        }
        if self.position.y - self.size/2.0 < 0.0 || self.position.y + self.size/2.0 > self.grid_size.y {
            self.velocity.y = -self.velocity.y;
        }

        // Clamp position to grid
        self.position.x = self.position.x.clamp(self.size/2.0, self.grid_size.x - self.size/2.0);
        self.position.y = self.position.y.clamp(self.size/2.0, self.grid_size.y - self.size/2.0);
    }
}