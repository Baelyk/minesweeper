extern crate piston_window;

use grid::piston_window::draw_state::DrawState;
use grid::piston_window::G2d;
use grid::piston_window::line::Line;
use grid::piston_window::types::Matrix2d;

struct Cell {
    x: usize,
    y: usize
}

struct Grid {
    width: usize,
    height: usize,
    cell_width: usize,
    cell_height: usize,
    line_radius: f64,
    line_color: [f64; 4]
}

impl Grid {
    pub fn new (width: usize, height: usize, cell_width: usize, cell_height: usize, line_radius: f64, line_color: [f64; 4]) -> Grid {
        Grid {
            width: width,
            height: height,
            cell_width: cell_width,
            cell_height: cell_height,
            line_radius: line_radius,
            line_color: line_color
        }
    }
    pub fn new_reg (size: usize, pixels: usize, line_radius: f64, line_color: [f64; 4]) -> Grid {
        Grid {
            width: size,
            height: size,
            cell_width: pixels,
            cell_height: pixels,
            line_radius: line_radius,
            line_color: line_color
        }
    }
    fn draw (&self, line: &Line, draw_state: &DrawState, transform: Matrix2d, g: &mut G2d) {
        for x in 0..self.width {
            line.draw([
                (x * self.cell_width) as f64,
                0.0,
                (x * self.cell_width) as f64,
                (self.height * self.cell_height) as f64
            ], draw_state, transform, g);
        }
        for y in 0..self.height {
            line.draw([
                0.0,
                (y * self.cell_height) as f64,
                (self.width * self.cell_width) as f64,
                (y * self.cell_height) as f64
            ], draw_state, transform, g);
        }
    }
}
