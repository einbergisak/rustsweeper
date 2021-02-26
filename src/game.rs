use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use ggez::{
    graphics::{spritebatch::SpriteBatch, Image},
    Context,
};

/// 2-Dimensional vector of tiles.
/// The outer vector is a vector of the game columns.
/// Tiles at coordinates (x, y) can be accessed via `TileArray[x][y]`
type TileArray = Vec<Vec<Tile>>;

/// Game tile
#[derive(Copy, Clone, Debug)]
pub(crate) struct Tile {
    pub(crate) is_revealed: bool,
    is_a_mine: bool,
    pub(crate) is_flagged: bool,
    pub(crate) number: Option<u8>,
}
impl Default for Tile {
    /// Creates a new, uninitialized tile with no 'active values'
    fn default() -> Self {
        Tile {
            is_revealed: false,
            is_a_mine: false,
            is_flagged: false,
            number: None,
        }
    }
}

/// Contains the data relevant to the game
pub(crate) struct GameContainer {
    pub(crate) tile_array: Box<TileArray>,
    tiles_revealed: usize,
    pub(crate) game_rows: usize,
    pub(crate) game_cols: usize,
    game_mines: usize,
    pub(crate) sprite_batch: SpriteBatch,
    pub(crate) scaled_tile_size: f32,
}
impl GameContainer {
    /// Creates a new game with the provided settings and seed.
    // TODO: Första rutan man klickar på och de runtomkring kan inte vara minor.
    pub(crate) fn new(
        mut ctx: &mut Context,
        game_rows: usize,
        game_cols: usize,
        game_mines: usize,
        seed: String,
        scaled_tile_size: f32,
    ) -> GameContainer {
        let mut hasher = DefaultHasher::default();
        let mut tile_array: Box<TileArray> = Box::new(vec![vec![Tile::default(); game_rows]; game_cols]);
        let mut hash;
        let mut current_number_of_mines: usize = 0;
        let (mut x_index, mut y_index) = (0, 0);

        // Uses the MAD-hashing technique to distribute the mines on the game field.
        'outer: loop {
            'inner: loop {
                seed.hash(&mut hasher);
                hash = hasher.finish();
                let mut tile = &mut tile_array[x_index][y_index];
                if (hash.overflowing_mul(23).0 + 7) % 53 == 0 && !tile.is_a_mine {
                    tile.is_a_mine = true;
                    current_number_of_mines += 1;
                    if current_number_of_mines == game_mines {
                        break 'outer;
                    }
                }
                y_index += 1;
                if y_index == game_rows {
                    y_index = 0;
                    break 'inner;
                }
            }
            x_index += 1;
            if x_index == game_cols {
                x_index = 0;
            }
        }
        let img = Image::new(&mut ctx, "/spritesheet.png").expect("Image loading error");
        let mut gc = GameContainer {
            tile_array,
            tiles_revealed: 0,
            game_rows,
            game_cols,
            game_mines,
            sprite_batch: SpriteBatch::new(img),
            scaled_tile_size,
        };
        for x in 0..game_cols {
            for y in 0..game_rows {
                gc.set_tile_number((x, y)); // Set tile number (the amount of nearby mines)
            }
        }

        gc
    }

    /// Reveals the tile at the given coordinates.
    pub(crate) fn reveal_tile_at(&mut self, (tile_x, tile_y): (usize, usize)) {
        let tile = &self.tile_array[tile_x][tile_y];
        if !tile.is_revealed {
            if tile.is_a_mine {
                self.lose(&tile);
                return;
            } else {
                self.reveal_nearby((tile_x, tile_y));
                self.check_if_win();
            }
        }
    }

    /// Toggles if the tile is flagged or not
    pub(crate) fn toggle_flag_at(&mut self, (tile_x, tile_y): (usize, usize)) {
        if !self.tile_array[tile_x][tile_y].is_revealed{
            // Toggles with bitwise XOR
            self.tile_array[tile_x][tile_y].is_flagged ^= true;
        }
    }

    pub(crate) fn chord_at(&mut self, (tile_x, tile_y): (usize, usize)) {
        todo!()
    }

    /// Recursively reveals tiles around the argument tile if it is empty `(is_a_mine == False && number == None)`
    fn reveal_nearby(&mut self, (tile_x, tile_y): (usize, usize)) {
        let mut tile: &mut Tile = &mut self.tile_array[tile_x][tile_y];
        if !tile.is_revealed && !tile.is_flagged {
            match tile.number {
                None if tile.is_a_mine => {
                    return;
                }
                None => {
                    tile.is_revealed = true;
                    self.map_tile_and_surrounding((tile_x, tile_y), GameContainer::reveal_nearby);
                }
                Some(_) => {
                    tile.is_revealed = true;
                }
            }
            self.tiles_revealed += 1;
        }
    }

    /// Sets the number for each non-mine tile
    fn set_tile_number(&mut self, (tile_x, tile_y): (usize, usize)) {
        if !self.tile_array[tile_x][tile_y].is_a_mine {
            let mut acc: u8 = 0; // +1 for each mine surrounding the argument tile
            let add_if_mine = |sself: &mut Self, (x, y): (usize, usize)| {
                if sself.tile_array[x][y].is_a_mine {
                    acc += 1;
                }
            };
            self.map_tile_and_surrounding((tile_x, tile_y), add_if_mine);
            let mut tile: &mut Tile = &mut self.tile_array[tile_x][tile_y];
            if acc != 0 {
                tile.number = Some(acc);
            }
        }
    }

    /// A higher-order function that applies the given function to the given tile and to all of the tiles surrounding it.
    fn map_tile_and_surrounding<T>(&mut self, (tile_x, tile_y): (usize, usize), mut func: T)
    where
        T: FnMut(&mut Self, (usize, usize)),
    {
        for xd in -1..=1 {
            for yd in -1..=1 {
                // Makes sure that it doesn't go outside of the tile_array index bounds
                let (x, y) = (tile_x as isize + xd, tile_y as isize + yd);
                if x >= 0 && x < self.game_cols as isize && y >= 0 && y < self.game_rows as isize {
                    func(self, (x as usize, y as usize))
                }
            }
        }
    }

    fn lose(&self, tile: &Tile) {
        todo!()
    }

    fn check_if_win(&self) {
        if self.tiles_revealed == self.game_cols * self.game_rows - self.game_mines {
            todo!()
        }
    }
}
