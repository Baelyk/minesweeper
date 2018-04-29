extern crate piston_window;

use std::fmt;
use piston_window::*;

struct Grid {
    width: usize,
    height: usize,
    pixels_x: usize,
    pixels_y: usize,
    line_width: f64,
    line_color: [f64; 4]
}
