extern crate piston_window;

mod grid;

// TODO: Try to limit this from a * to just the things we need
use piston_window::*;
use grid::Grid;

const TILE_SIZE: usize = 50;
const GRID_X: usize = 10;
const GRID_Y: usize = 10;
const GRID_RADIUS: f64 = 1.0;
const GRID_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const BACK_COLOR: [f32; 4] = [0.55, 0.7, 0.95, 1.0];
const GRID_OFFSET: usize = 100;

// TODO: Hover color, clicked color (held down but not released)

fn main() {
    // Title: Minesweeper
    // GRID_X * TILE_SIZE by GRID_Y * TILE_SIZE
    // esc to exit
    // not resizable
    let mut window: PistonWindow = WindowSettings::new("Minesweeper", [GRID_X as u32 * TILE_SIZE as u32 + GRID_OFFSET as u32 * 2 as u32, GRID_Y as u32 * TILE_SIZE as u32 + GRID_OFFSET as u32 * 2 as u32])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    // Set the ups to 1 by default, but allow it to be changed
    // TODO: determine if ups needs to be mutable
    let mut ups = 1;
    let grid = Grid::new(GRID_X, GRID_Y, TILE_SIZE, TILE_SIZE, GRID_RADIUS, GRID_COLOR, BACK_COLOR);

    window.events.set_ups(ups);

    while let Some(event) = window.next() {
        // Draw the grid
        window.draw_2d(&event, |c, g| {
            clear([1.0; 4], g); // [1.0; 4] is short for [1.0, 1.0, 1.0, 1.0] which is white
            // Draw the grid with a Noned &DrawState
            grid.draw_offset(GRID_OFFSET as f64, GRID_OFFSET as f64, &DrawState {
                scissor: None,
                stencil: None,
                blend: None
            }, c.transform, g);
        });

        // Let the ups be changed by the up and down arrow keys
        if let Some(Button::Keyboard(Key::Up)) = event.press_args() {
            ups += 1;
            window.events.set_ups(ups);
        }
        if let Some(Button::Keyboard(Key::Down)) = event.press_args() {
            if ups > 1 { ups -= 1; }
            window.events.set_ups(ups);
        }
    }
}
