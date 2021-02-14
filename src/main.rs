

const GAME_WIDTH: usize = 16;
const GAME_HEIGHT: usize = 16;


struct Tile{
    is_revealed: bool,
    is_bomb: bool,
    number: u8,
    coordinates: (usize, usize),
}

struct GameContainer{
    tile_rows: [[Tile; GAME_WIDTH]; GAME_HEIGHT]
}

impl GameContainer{
    fn reveal_tile(&mut self, &(x, y): (usize, usize)) {
        let mut tile = &mut self.tile_rows[y][x];
        if !tile.is_revealed{
            if tile.is_bomb{
                self.lose(&tile);
            }else{
                tile.is_revealed = true;
                if tile.number == 0 {
                    self.reveal_nearby(&tile)
                }
            }
        }
    }

}



fn main() {





}
