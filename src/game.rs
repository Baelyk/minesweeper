#![allow(dead_code)] // TODO: Remove this!!!
extern crate rand; // TODO: Pick a spelling of neighbours

use self::rand::{thread_rng, Rng};

#[derive(Copy, Clone, PartialEq)]
enum TileType {
    Mine,
    Empty(usize), // usize representing number of neighboring mines
}

#[derive(Copy, Clone)]
struct GameTile {
    x: usize,
    y: usize,
    tile: TileType,
    flagged: bool,
    revealed: bool,
}

struct Game {
    width: usize,
    height: usize,
    board: Vec<Vec<GameTile>>,
    mines: usize,  // Number of mines in the game
    solved: usize, // Number of mines flagged
    flags: usize,  // Number of tiles flagged (Empty and Mine)
    over: bool,
}

impl Game {
    pub fn new(width: usize, height: usize, mines: usize) -> Game {
        let mut board: Vec<Vec<GameTile>> = vec![
            vec![
                GameTile {
                    x: 0,
                    y: 0,
                    tile: TileType::Empty(0),
                    flagged: false,
                    revealed: false,
                };
                height
            ];
            width
        ];
        let mut mine_array: Vec<(usize, usize)> = vec![(0, 0); mines];
        let mut rng = thread_rng();
        for i in 0..mines {
            mine_array[i] = rng.gen::<(usize, usize)>();
        }

        for x in 0..width {
            for y in 0..height {
                let mut tiletype = TileType::Empty(0);
                if mine_array.contains(&(x, y)) {
                    tiletype = TileType::Mine;
                }

                board[x][y] = GameTile {
                    x: x,
                    y: y,
                    tile: tiletype,
                    flagged: false,
                    revealed: false,
                }
            }
        }

        Game {
            width: width,
            height: height,
            board: board,
            mines: mines,
            flags: 0,
            solved: 0,
            over: false,
        }.init()
    }
    pub fn get_tile(&self, x: usize, y: usize) -> GameTile {
        self.board[x][y]
    }
    pub fn reveal_tile(&mut self, x: usize, y: usize) {
        let mut tile = self.get_tile(x, y);
        // Trying to reveal a flagged or revealed tile is not allowed
        if tile.flagged {
            panic!("attempted to reveal flagged tile ({}, {})", x, y)
        }
        if tile.revealed {
            panic!("attempted to reveal already revealed tile ({}, {})", x, y)
        }

        tile.revealed = true;

        // If tile is empty and not next to any mines
        if match tile.tile {
            TileType::Mine => {
                self.lose();
                false
            }
            TileType::Empty(count) => count == 0,
        } {
            for neighbor in tile.neighbors(self).iter() {
                // If neighbor is not revealed and not flagged
                if !neighbor.revealed && !neighbor.flagged {
                    // If neighbor is an empty tile
                    if match neighbor.tile {
                        TileType::Mine => false,
                        TileType::Empty(_) => true,
                    } {
                        // Reveal neighbor
                        self.reveal_tile(neighbor.x, neighbor.y)
                    }
                }
            }
        }
    }
    pub fn flag_tile(&mut self, x: usize, y: usize) {
        let mut tile = self.get_tile(x, y);
        // Trying to flag a flagged or revealed tile is not allowed
        if tile.flagged {
            panic!("attempted to flag already flagged tile ({}, {})", x, y)
        }
        if tile.revealed {
            panic!("attempted to flag revealed tile ({}, {})", x, y)
        }

        tile.flagged = true;
        match tile.tile {
            TileType::Mine => {
                self.flags += 1;
                self.solved += 1;
            }
            TileType::Empty(_) => {
                self.flags += 1;
            }
        };

        if self.mines == self.solved {
            // TODO: self.win();
        }
    }
    pub fn unflag_tile(&mut self, x: usize, y: usize) {
        let mut tile = self.get_tile(x, y);
        // Trying to unflag an unflagged or revealed tile is not allowed
        if tile.flagged {
            panic!("attempted to unflag not flagged tile ({}, {})", x, y)
        }
        if tile.revealed {
            panic!("attempted to unflag revealed tile ({}, {})", x, y)
        }

        tile.flagged = false;
        match tile.tile {
            TileType::Mine => {
                self.flags -= 1;
                self.solved -= 1;
            }
            TileType::Empty(_) => {
                self.flags -= 1;
            }
        };
    }
    pub fn win(&mut self) {
        println!("Good game!");
        self.over = true;
    }
    pub fn lose(&mut self) {
        println!("Game over :(");
        self.over = true;
    }
    pub fn init(self) -> Game {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut tile = self.get_tile(x, y);
                if match tile.tile {
                    TileType::Mine => false,
                    TileType::Empty(_) => true,
                } {
                    let mut count = 0;
                    for neighbor in tile.neighbors(&self).iter() {
                        match neighbor.tile {
                            TileType::Mine => count += 1,
                            TileType::Empty(_) => count += 0,
                        }
                    }
                    tile.tile = TileType::Empty(count);
                }
            }
        }

        self
    }
}

impl GameTile {
    pub fn flag(&mut self) {
        self.flagged = true;
    }
    pub fn unflag(&mut self) {
        self.flagged = false;
    }
    pub fn new_empty() -> GameTile {
        GameTile {
            x: 0,
            y: 0,
            tile: TileType::Empty(0),
            flagged: false,
            revealed: false,
        }
    }
    pub fn neighbors(&self, game: &Game) -> [GameTile; 8] {
        let mut neighbours_pos = [(self.x, self.y); 8];
        let mut neighbours = [*self; 8];
        const POSITIONS: [(i32, i32); 8] = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        for i in 0..8 {
            let (x, y) = POSITIONS[i];
            if self.x as i32 + x >= 0 && self.y as i32 + y >= 0
                && self.x as i32 + x < game.width as i32
                && self.y as i32 + y < game.height as i32
            {
                neighbours_pos[i] = ((self.x as i32 + x) as usize, (self.y as i32 + y) as usize);
            }
            neighbours[i] = game.get_tile(x as usize, y as usize);
        }

        neighbours
    }
}
