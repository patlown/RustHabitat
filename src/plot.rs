use ggez::graphics::{self, Color, DrawMode, Mesh, Rect, Text};
use ggez::glam::Vec2;
use ggez::{Context, GameResult};
use crate::entity::{Entity, EntityType};

const MAX_DATA_POINTS: usize = 100;

pub struct Plot {
    position: Vec2,
    size: Vec2,
    entity_filter: Box<dyn Fn(&Entity) -> bool>,
    data_points: Vec<f32>,
    title: String,
}

impl Plot {
    pub fn new<F>(position: Vec2, size: Vec2, title: String, entity_filter: F) -> Self
    where
        F: Fn(&Entity) -> bool + 'static,
    {
        Plot {
            position,
            size,
            entity_filter: Box::new(entity_filter),
            data_points: Vec::new(),
            title,
        }
    }

    pub fn update(&mut self, entities: &[Entity]) {
        let count = entities.iter().filter(|e| (self.entity_filter)(e)).count() as f32;
        self.data_points.push(count);
        if self.data_points.len() > MAX_DATA_POINTS {
            self.data_points.remove(0);
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let plot_rect = Rect::new(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y
        );

        // Draw plot background
        let plot_background = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            plot_rect,
            Color::WHITE,
        )?;
        canvas.draw(&plot_background, Vec2::new(0.0, 0.0));

        // Draw plot border
        let plot_border = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            plot_rect,
            Color::BLACK,
        )?;
        canvas.draw(&plot_border, Vec2::new(0.0, 0.0));

        // Draw title
        let title_text = Text::new(&self.title);
        let text_pos = Vec2::new(
            self.position.x + self.size.x / 2.0,
            self.position.y + 20.0
        );
        canvas.draw(
            &title_text,
            graphics::DrawParam::default()
                .color(Color::BLACK)
                .dest(text_pos)
                .offset(Vec2::new(0.5, 0.5))
        );

        // Draw the actual plot line
        if self.data_points.len() > 1 {
            let max_value = self.data_points.iter().cloned().fold(0.0/0.0, f32::max).max(1.0);
            let points: Vec<Vec2> = self.data_points.iter().enumerate().map(|(i, &value)| {
                Vec2::new(
                    self.position.x + (i as f32 / (self.data_points.len() - 1) as f32) * self.size.x,
                    self.position.y + self.size.y - (value / max_value) * (self.size.y - 40.0)
                )
            }).collect();

            let line = Mesh::new_line(ctx, &points, 2.0, Color::BLUE)?;
            canvas.draw(&line, Vec2::new(0.0, 0.0));
        }

        Ok(())
    }
}