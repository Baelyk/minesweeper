#![allow(dead_code)] // TODO: Remove this!!!
extern crate rand; // TODO: Pick a spelling of neighbours

use self::rand::{thread_rng, Rng};

#[derive(Copy, Clone, Debug)]
pub enum TileType {
    Mine,
    Empty(usize), // usize representing number of neighboring mines
}

#[derive(Copy, Clone, Debug)]
pub struct GameTile {
    x: usize,
    y: usize,
    pub tile: TileType,
    pub flagged: bool,
    pub revealed: bool,
}

pub struct Game {
    width: usize,
    height: usize,
    board: Vec<Vec<GameTile>>,
    mines: usize,  // Number of mines in the game
    solved: usize, // Number of mines flagged
    flags: usize,  // Number of tiles flagged (Empty and Mine)
    pub state: usize,
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
            mine_array[i] = (rng.gen_range(0, width), rng.gen_range(0, height));
        }

        println!("{:?}", mine_array);

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
            state: 1,
        }.init()
    }
    pub fn get_tile(&self, x: usize, y: usize) -> GameTile {
        self.board[x][y]
    }
    pub fn reveal_tile(&mut self, x: usize, y: usize) {
        // TODO: Change this to take a GameTile as input, not its coords
        let mut tile = self.get_tile(x, y);
        // println!("o{:?}", tile);
        // Trying to reveal a flagged or revealed tile is not allowed
        if tile.flagged {
            panic!("attempted to reveal flagged tile ({}, {})", x, y)
        }
        if tile.revealed {
            panic!("attempted to reveal already revealed tile ({}, {})", x, y)
        }

        tile.revealed = true;
        self.board[x][y] = tile;
        // println!("u{:?}", tile);

        // println!("{:?}", tile.neighbors(self));

        // If tile is empty and not next to any mines
        if match tile.tile {
            TileType::Mine => {
                self.lose();
                false
            }
            TileType::Empty(count) => count == 0,
        } {
            // println!("My TileType is {:?}", tile.tile);
            for neighbor in tile.neighbors(self).iter() {
                // println!("x{:?}", neighbor);
                // If neighbor is not revealed and not flagged
                /*println!(
                    "!({} == {}: {} && {} == {}: {}) --- {}",
                    neighbor.x,
                    tile.x,
                    neighbor.x == tile.x,
                    neighbor.y,
                    tile.y,
                    neighbor.y == tile.y,
                    !(neighbor.x == tile.x && neighbor.y == tile.y)
                );*/
                if !(neighbor.x == tile.x && neighbor.y == tile.y) {
                    if !self.board[neighbor.x][neighbor.y].revealed
                        && !self.board[neighbor.x][neighbor.y].flagged
                    {
                        // use self.board[x][y] to ensure that the tile hasn't been updated "elsewhere" (previously in recursions)
                        // println!("hi");
                        // If neighbor is an empty tile
                        match neighbor.tile {
                            TileType::Mine => {
                                /*println!(
                                "Lol don't reveal me I'm a mine! ({}, {})",
                                neighbor.x, neighbor.y
                            )*/
                            }
                            TileType::Empty(_) => {
                                // TODO: PANIC IF A MINE IS TOLD TO BE REVEALD <------------------------------------------------------------------>
                                // println!("n{:?}", neighbor);
                                // Reveal neighbor
                                self.reveal_tile(neighbor.x, neighbor.y);
                            }
                        };
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
        self.board[x][y] = tile;
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
        if !tile.flagged {
            panic!("attempted to unflag not flagged tile ({}, {})", x, y)
        }
        if tile.revealed {
            panic!("attempted to unflag revealed tile ({}, {})", x, y)
        }

        tile.flagged = false;
        self.board[x][y] = tile;
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
        self.state = 2;
    }
    pub fn lose(&mut self) {
        println!("Game over :(");
        self.state = 0;
    }
    pub fn init(mut self) -> Game {
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
                    self.board[x][y] = tile;
                }
            }
        }

        self
    }
}

impl GameTile {
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
            neighbours[i] =
                game.get_tile(neighbours_pos[i].0 as usize, neighbours_pos[i].1 as usize);
        }
        // println!("{:?}", neighbours_pos);
        neighbours
    }
}
