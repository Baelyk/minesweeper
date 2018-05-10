#![allow(dead_code)]

extern crate piston_window;

use piston_window::character::CharacterCache;
use grid::piston_window::draw_state::DrawState;
use grid::piston_window::G2d;
use grid::piston_window::Graphics;
use grid::piston_window::line::Line;
use grid::piston_window::rectangle::Rectangle;
use grid::piston_window::text::Text;
use grid::piston_window::math::Matrix2d;
use piston_window::Transformed;

pub struct GridCell {
    pub x: usize,
    pub y: usize,
    width: usize,
    height: usize,
    x_off: usize,
    y_off: usize,
}

pub struct Grid {
    width: usize,
    height: usize,
    cell_width: usize,
    cell_height: usize,
    line_radius: f64,
    line_color: [f32; 4],
    background_color: [f32; 4],
    x_off: usize,
    y_off: usize,
}

impl GridCell {
    pub fn color(
        self,
        color: [f32; 4],
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G2d,
    ) -> GridCell {
        let rect = Rectangle::new(color);
        rect.draw(
            [
                // TODO: Change these values from the temporary 1s
                (self.x * self.width + self.x_off + 1) as f64,
                (self.y * self.height + self.y_off + 1) as f64,
                (self.width - 1) as f64,
                (self.height - 1) as f64,
            ],
            draw_state,
            transform,
            g,
        );

        self
    }
    pub fn text<C, G>(
        self,
        color: [f32; 4],
        font_size: u32,
        text: &str,
        cache: &mut C,
        transform: Matrix2d,
        g: &mut G,
    ) -> GridCell
    where
        C: CharacterCache,
        G: Graphics<Texture = <C as CharacterCache>::Texture>,
    {
        Text::new_color(color, font_size).draw(
            text,
            cache,
            &Default::default(),
            transform.trans(
                (self.x * self.width + self.x_off + self.width / 2) as f64,
                (self.y * self.height + self.y_off + self.height) as f64,
            ),
            g,
        );

        self
    }
}

impl Grid {
    pub fn new(
        width: usize,
        height: usize,
        cell_width: usize,
        cell_height: usize,
        line_radius: f64,
        line_color: [f32; 4],
        background_color: [f32; 4],
        x_off: usize,
        y_off: usize,
    ) -> Grid {
        Grid {
            width: width,
            height: height,
            cell_width: cell_width,
            cell_height: cell_height,
            line_radius: line_radius,
            line_color: line_color,
            background_color: background_color,
            x_off: x_off,
            y_off: y_off,
        }
    }
    #[allow(dead_code)]
    pub fn new_reg(
        size: usize,
        pixels: usize,
        line_radius: f64,
        line_color: [f32; 4],
        background_color: [f32; 4],
        off: usize,
    ) -> Grid {
        Grid {
            width: size,
            height: size,
            cell_width: pixels,
            cell_height: pixels,
            line_radius: line_radius,
            line_color: line_color,
            background_color: background_color,
            x_off: off,
            y_off: off,
        }
    }
    #[allow(dead_code)]
    pub fn draw(&self, draw_state: &DrawState, transform: Matrix2d, g: &mut G2d) {
        let line = Line::new(self.line_color, self.line_radius);
        let rect = Rectangle::new(self.background_color);
        rect.draw(
            [
                0.0 + self.x_off as f64,
                0.0 + self.y_off as f64,
                (self.width * self.cell_width) as f64,
                (self.height * self.cell_height) as f64,
            ],
            draw_state,
            transform,
            g,
        );

        for x in 0..self.width + 1 {
            line.draw(
                [
                    (x * self.cell_width) as f64 + self.x_off as f64,
                    0.0 + self.y_off as f64,
                    (x * self.cell_width) as f64 + self.x_off as f64,
                    (self.height * self.cell_height) as f64 + self.y_off as f64,
                ],
                draw_state,
                transform,
                g,
            );
        }
        for y in 0..self.height + 1 {
            line.draw(
                [
                    0.0 + self.x_off as f64,
                    (y * self.cell_height) as f64 + self.y_off as f64,
                    (self.width * self.cell_width) as f64 + self.x_off as f64,
                    (y * self.cell_height) as f64 + self.y_off as f64,
                ],
                draw_state,
                transform,
                g,
            );
        }
    }
    pub fn select_cell(&self, x: usize, y: usize) -> GridCell {
        let x_coord = (x - self.x_off - x % self.cell_width) / self.cell_width;
        let y_coord = (y - self.y_off - y % self.cell_height) / self.cell_height;
        self.get_cell(x_coord, y_coord)
    }
    pub fn get_cell(&self, x: usize, y: usize) -> GridCell {
        GridCell {
            x: x,
            y: y,
            width: self.cell_width,
            height: self.cell_height,
            x_off: self.x_off,
            y_off: self.y_off,
        }
    }
}
