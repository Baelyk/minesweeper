extern crate piston_window;

use grid::piston_window::draw_state::DrawState;
use grid::piston_window::G2d;
use grid::piston_window::line::Line;
use grid::piston_window::rectangle::Rectangle;
use grid::piston_window::types::Matrix2d;

#[allow(dead_code)]
pub struct Cell {
    x: usize,
    y: usize
}

pub struct Grid {
    width: usize,
    height: usize,
    cell_width: usize,
    cell_height: usize,
    line_radius: f64,
    line_color: [f32; 4],
    background_color: [f32; 4]
}

impl Grid {
    pub fn new (width: usize, height: usize, cell_width: usize, cell_height: usize, line_radius: f64, line_color: [f32; 4], background_color: [f32; 4]) -> Grid {
        Grid {
            width: width,
            height: height,
            cell_width: cell_width,
            cell_height: cell_height,
            line_radius: line_radius,
            line_color: line_color,
            background_color: background_color
        }
    }
    #[allow(dead_code)]
    pub fn new_reg (size: usize, pixels: usize, line_radius: f64, line_color: [f32; 4], background_color: [f32; 4]) -> Grid {
        Grid {
            width: size,
            height: size,
            cell_width: pixels,
            cell_height: pixels,
            line_radius: line_radius,
            line_color: line_color,
            background_color: background_color
        }
    }
    #[allow(dead_code)]
    pub fn draw (&self, draw_state: &DrawState, transform: Matrix2d, g: &mut G2d) {
        let line = Line::new(self.line_color, self.line_radius);
        let rect = Rectangle::new(self.background_color);
        rect.draw([0.0, 0.0, (self.width * self.cell_width) as f64, (self.height * self.cell_height) as f64], draw_state, transform, g);

        for x in 0..self.width + 1 {
            line.draw([
                (x * self.cell_width) as f64,
                0.0,
                (x * self.cell_width) as f64,
                (self.height * self.cell_height) as f64
            ], draw_state, transform, g);
        }
        for y in 0..self.height + 1 {
            line.draw([
                0.0,
                (y * self.cell_height) as f64,
                (self.width * self.cell_width) as f64,
                (y * self.cell_height) as f64
            ], draw_state, transform, g);
        }
    }
    pub fn draw_offset (&self, x_off: f64, y_off: f64, draw_state: &DrawState, transform: Matrix2d, g: &mut G2d) {
        let line = Line::new(self.line_color, self.line_radius);
        let rect = Rectangle::new(self.background_color);
        rect.draw([0.0 + x_off, 0.0 + y_off, (self.width * self.cell_width) as f64, (self.height * self.cell_height) as f64], draw_state, transform, g);

        for x in 0..self.width + 1 {
            line.draw([
                (x * self.cell_width) as f64 + x_off,
                0.0 + y_off,
                (x * self.cell_width) as f64 + x_off,
                (self.height * self.cell_height) as f64 + y_off
            ], draw_state, transform, g);
        }
        for y in 0..self.height + 1 {
            line.draw([
                0.0 + x_off,
                (y * self.cell_height) as f64 + y_off,
                (self.width * self.cell_width) as f64 + x_off,
                (y * self.cell_height) as f64 + y_off
            ], draw_state, transform, g);
        }
    }
}
