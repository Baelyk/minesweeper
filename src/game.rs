#![allow(dead_code)] // TODO: Remove this!!!
extern crate rand; // TODO: Pick a spelling of neighbours

use self::rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
enum TileType {
    Mine,
    Empty(usize) // usize representing number of neighboring mines
}

#[derive(Copy, Clone)]
struct GameTile {
    x: usize,
    y: usize,
    tile: TileType,
    flagged: bool
}

struct Game {
    width: usize,
    height: usize,
    board: Vec<Vec<GameTile>>,
    mines: usize, // Number of mines in the game
}

impl Game {
    pub fn new (width: usize, height: usize, mines: usize) -> Game {
        let mut board: Vec<Vec<GameTile>> = vec![vec![GameTile {x: 0, y: 0, tile: TileType::Empty(0), flagged: false}; height]; width];
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
                    flagged: false
                }
            }
        }

        Game {
            width: width,
            height: height,
            board: board,
            mines: mines
        }
    }
    pub fn get_tile (&self, x: usize, y: usize) -> GameTile {
        self.board[x][y]
    }
}

impl GameTile {
    pub fn new_empty () -> GameTile {
        GameTile {
            x: 0,
            y: 0,
            tile: TileType::Empty(0),
            flagged: false
        }
    }
    pub fn neighbors (&self, game: &Game) -> [GameTile; 8] {
        let mut neighbours_pos = [(self.x, self.y); 8];
        let mut neighbours = [*self; 8];
        const POSITIONS: [(i32, i32); 8] = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];

        for i in 0..8 {
            let (x, y) = POSITIONS[i];
            if self.x as i32 + x >= 0
                && self.y as i32 + y >= 0
                && self.x as i32 + x < game.width as i32
                && self.y as i32 + y < game.height as i32 {
                neighbours_pos[i] = ((self.x as i32 + x) as usize, (self.y as i32 + y) as usize);
            }
            neighbours[i] = game.get_tile(x as usize, y as usize);
        }

        neighbours
    }
}
