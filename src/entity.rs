use ggez::graphics::{self, Color, Mesh, DrawMode};
use ggez::glam::Vec2;
use ggez::{Context, GameResult};
use rand::Rng;

pub struct Entity {
    position: Vec2,
    velocity: Vec2,
    size: f32,
    color: Color,
    grid_size: Vec2,
    eye_offset: Vec2,
}

impl Entity {
    pub fn new(x: f32, y: f32, size: f32, color: Color, grid_width: f32, grid_height: f32) -> Self {
        Entity {
            position: Vec2::new(x, y),
            velocity: Vec2::new(0.0, 0.0),
            size,
            color,
            grid_size: Vec2::new(grid_width, grid_height),
            eye_offset: Vec2::new(0.0, 0.0),
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        // Draw body
        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            self.position,
            self.size / 2.0,
            0.1,
            self.color,
        )?;
        canvas.draw(&circle, graphics::DrawParam::default());

        // Draw eyes
        let eye_size = self.size * 0.3;
        let eye_distance = self.size * 0.2;
        let left_eye_pos = self.position + Vec2::new(-eye_distance, -eye_distance);
        let right_eye_pos = self.position + Vec2::new(eye_distance, -eye_distance);

        // Eye whites
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

        // Eye pupils
        let pupil_size = eye_size * 0.6;
        let pupil = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            self.eye_offset,
            pupil_size / 2.0,
            0.1,
            Color::BLACK,
        )?;
        canvas.draw(&pupil, graphics::DrawParam::new().dest(left_eye_pos));
        canvas.draw(&pupil, graphics::DrawParam::new().dest(right_eye_pos));

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

        // Update eye offset for googly effect
        // let max_eye_offset = self.size * 0.025;
        // self.eye_offset = Vec2::new(
        //     rng.gen_range(-max_eye_offset..max_eye_offset),
        //     rng.gen_range(-max_eye_offset..max_eye_offset),
        // );
    }
}