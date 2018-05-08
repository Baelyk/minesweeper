extern crate piston_window;

mod game;
mod grid;

// TODO: Try to limit this from a * to just the things we need
use piston_window::*;
use grid::Grid;
use game::{Game, TileType};

const TILE_SIZE: usize = 50;
const GRID_X: usize = 10;
const GRID_Y: usize = 10;
const GRID_RADIUS: f64 = 1.0;
const GRID_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const BACK_COLOR: [f32; 4] = [0.55, 0.7, 0.95, 1.0];
const REVEALED_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
const GRID_OFFSET: usize = 100;
const MINES: usize = 20;

// TODO: Hover color, clicked color (held down but not released)

fn main() {
    // Title: Minesweeper
    // GRID_X * TILE_SIZE by GRID_Y * TILE_SIZE
    // esc to exit
    // not resizable
    let mut window: PistonWindow = WindowSettings::new(
        "Minesweeper",
        [
            GRID_X as u32 * TILE_SIZE as u32 + GRID_OFFSET as u32 * 2 as u32,
            GRID_Y as u32 * TILE_SIZE as u32 + GRID_OFFSET as u32 * 2 as u32,
        ],
    ).exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    let grid = Grid::new(
        GRID_X,
        GRID_Y,
        TILE_SIZE,
        TILE_SIZE,
        GRID_RADIUS,
        GRID_COLOR,
        BACK_COLOR,
        GRID_OFFSET,
        GRID_OFFSET,
    );
    let mut game = Game::new(GRID_X, GRID_Y, MINES);
    let mut mouse: (usize, usize) = (0, 0);
    let mut update = true;
    // Set the ups to 1 by default, but allow it to be changed
    // TODO: determine if ups needs to be mutable
    let mut ups = 1;
    let mut focused = false;

    window.events.set_ups(ups);

    while let Some(event) = window.next() {
        // Draw the grid
        window.draw_2d(&event, |c, g| {
            clear([1.0; 4], g); // [1.0; 4] is short for [1.0, 1.0, 1.0, 1.0] which is white
                                // Draw the grid with a Noned &DrawState
            grid.draw(
                &DrawState {
                    scissor: None,
                    stencil: None,
                    blend: None,
                },
                c.transform,
                g,
            );

            for x in 0..GRID_X as usize {
                for y in 0..GRID_Y as usize {
                    let tile = game.get_tile(x, y);
                    // println!("{:?}", tile);
                    match tile.tile {
                        TileType::Mine => {
                            /*grid.get_cell(x, y).color(
                            [1.0, 0.0, 0.0, 1.0],
                            &DrawState {
                                scissor: None,
                                stencil: None,
                                blend: None,
                            },
                            c.transform,
                            g,
                        )*/
                        }
                        TileType::Empty(_count) => {
                            if tile.revealed {
                                grid.get_cell(x, y).color(
                                    REVEALED_COLOR,
                                    &DrawState {
                                        scissor: None,
                                        stencil: None,
                                        blend: None,
                                    },
                                    c.transform,
                                    g,
                                )
                            }
                        }
                    }
                }
            }
        });

        // Let the ups be changed by the up and down arrow keys
        if let Some(Button::Keyboard(Key::Up)) = event.press_args() {
            ups += 1;
            window.events.set_ups(ups);
        }
        if let Some(Button::Keyboard(Key::Down)) = event.press_args() {
            if ups > 1 {
                ups -= 1;
            }
            window.events.set_ups(ups);
        }

        // Allow the game to be played with the mouse
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            if focused {
                let (x, y) = mouse;
                let cell = grid.select_cell(x, y);
                let tile = game.get_tile(cell.x, cell.y);
                println!("{:?}", tile);
                if !tile.revealed && !tile.flagged {
                    game.reveal_tile(cell.x, cell.y);
                }
                // println!("{:?}", cells[cell_x as usize][cell_y as usize]);
            }
        }
        event.mouse_cursor(|x, y| {
            if x > 0.0 && x < (GRID_X * TILE_SIZE) as f64 && y > 0.0
                && x < (GRID_Y * TILE_SIZE) as f64
            {
                focused = true;
                mouse = (x as usize, y as usize);
            } else {
                focused = false;
            }
        });

        // Update function
        event.update(|_| {
            if update {
                window.draw_2d(&event, |c, g| {
                    clear([1.0; 4], g); // [1.0; 4] is short for [1.0, 1.0, 1.0, 1.0] which is white
                                        // Draw the grid with a Noned &DrawState
                    grid.draw(
                        &DrawState {
                            scissor: None,
                            stencil: None,
                            blend: None,
                        },
                        c.transform,
                        g,
                    );

                    for x in 0..GRID_X as usize {
                        for y in 0..GRID_Y as usize {
                            let tile = game.get_tile(x, y);
                            println!("{:?}", tile);
                            if tile.revealed {
                                grid.get_cell(x, y).color(
                                    REVEALED_COLOR,
                                    &DrawState {
                                        scissor: None,
                                        stencil: None,
                                        blend: None,
                                    },
                                    c.transform,
                                    g,
                                )
                            }
                        }
                    }
                });
            }
        });
    }
}
