use ggez::graphics::{self, Color, DrawMode, Mesh, Rect, Text};
use ggez::glam::Vec2;
use ggez::{Context, GameResult};
use std::collections::VecDeque;
use crate::entity::{Entity, EntityType};

const WINDOW_DURATION: f32 = 30.0; // 30 seconds window

pub enum TrackingStat {
    Population,
}

pub struct AreaChart {
    position: Vec2,
    size: Vec2,
    color: Color,
    title: String,
    entity_type: EntityType,
    tracking_stat: TrackingStat,
    data: VecDeque<(f32, f32)>, // (time, value)
    max_value: f32,
    start_time: f32,
    background_color: Color, // Added background color field
}

impl AreaChart {
    pub fn new(position: Vec2, size: Vec2, color: Color, title: String, entity_type: EntityType, tracking_stat: TrackingStat) -> Self {
        AreaChart {
            position,
            size,
            color,
            title,
            entity_type,
            tracking_stat,
            data: VecDeque::new(),
            max_value: 1.0,
            start_time: 0.0,
            background_color: Color::new(0.15, 0.15, 0.15, 1.0), // Initialize background color to dark gray
        }
    }

    pub fn update(&mut self, time: f32, entities: &[Entity]) {
        if self.data.is_empty() {
            self.start_time = time;
        }

        let value = match self.tracking_stat {
            TrackingStat::Population => entities.iter()
                .filter(|e| e.entity_type() == self.entity_type)
                .count() as f32,
        };

        self.max_value = self.max_value.max(value);
        self.data.push_back((time, value));

        // Remove old data points
        while let Some((t, _)) = self.data.front() {
            if time - *t > WINDOW_DURATION {
                self.data.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        // Draw background
        let background = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(self.position.x, self.position.y, self.size.x, self.size.y),
            self.background_color,
        )?;
        canvas.draw(&background, Vec2::new(0.0, 0.0));

        // Draw title
        let title = Text::new(&self.title);
        canvas.draw(
            &title,
            graphics::DrawParam::default()
                .dest(Vec2::new(self.position.x + 5.0, self.position.y + 5.0))
                .color(Color::WHITE)
                .scale(Vec2::new(0.8, 0.8)),
        );

        if self.data.is_empty() {
            return Ok(());
        }

        // Calculate scales
        let x_scale = self.size.x / WINDOW_DURATION;
        let max_value = self.max_value * 1.1; // Leave 10% space at the top
        let y_scale = (self.size.y - 40.0) / max_value;  // Leave space for title and x-axis label

        // Prepare points for the area polygon
        let mut area_points = vec![Vec2::new(self.position.x, self.position.y + self.size.y - 20.0)];
        let mut line_points = Vec::new();

        let current_time = self.data.back().unwrap().0;
        let start_time = current_time - WINDOW_DURATION;

        for &(t, v) in &self.data {
            if t >= start_time {
                let x = self.position.x + (t - start_time) * x_scale;
                let y = self.position.y + self.size.y - 20.0 - v * y_scale;
                let point = Vec2::new(x, y);
                area_points.push(point);
                line_points.push(point);
            }
        }

        // Ensure the line extends to the right edge of the chart
        if let Some(last_point) = line_points.last().cloned() {
            let right_edge = Vec2::new(self.position.x + self.size.x, last_point.y);
            line_points.push(right_edge);
            area_points.push(right_edge);
        }

        // Add final point to complete the area
        area_points.push(Vec2::new(self.position.x + self.size.x, self.position.y + self.size.y - 20.0));

        // Draw area
        if area_points.len() >= 3 {
            let area = Mesh::new_polygon(
                ctx,
                DrawMode::fill(),
                &area_points,
                self.color,
            )?;
            canvas.draw(&area, Vec2::new(0.0, 0.0));
        }

        // Draw line
        if line_points.len() >= 2 {
            let line = Mesh::new_line(
                ctx,
                &line_points,
                2.0, // Increased line thickness
                Color::WHITE,
            )?;
            canvas.draw(&line, Vec2::new(0.0, 0.0));
        }

        // Draw y-axis label
        let y_label = Text::new("Population");
        canvas.draw(
            &y_label,
            graphics::DrawParam::default()
                .dest(Vec2::new(self.position.x, self.position.y + self.size.y / 2.0))
                .rotation(std::f32::consts::FRAC_PI_2)
                .color(Color::WHITE)
                .scale(Vec2::new(1.0, 1.0)),
        );

        // Draw x-axis label
        let x_label = Text::new("Time");
        canvas.draw(
            &x_label,
            graphics::DrawParam::default()
                .dest(Vec2::new(self.position.x + self.size.x / 2.0, self.position.y + self.size.y - 5.0))
                .color(Color::WHITE)
                .scale(Vec2::new(1.0, 1.0)),
        );

        // Draw current population count
        if let Some(&(_, current_value)) = self.data.back() {
            let count_text = Text::new(format!("Current: {:.0}", current_value));
            canvas.draw(
                &count_text,
                graphics::DrawParam::default()
                    .dest(Vec2::new(self.position.x + self.size.x - 60.0, self.position.y + 5.0))
                    .color(Color::WHITE)
                    .scale(Vec2::new(0.6, 0.6)),
            );
        }

        Ok(())
    }
}