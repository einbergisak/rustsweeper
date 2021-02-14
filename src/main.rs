const GAME_COLS: usize = 16;
const GAME_ROWS: usize = 16;


struct Coordinate{
    x: usize,
    y: usize
}

struct Tile{
    is_revealed: bool,
    is_bomb: bool,
    number: Option<u8>,
    coordinates: Coordinate,
}

struct GameContainer{
    tile_array: [[Tile; GAME_ROWS]; GAME_COLS]
}

impl GameContainer{
    fn reveal_tile(&mut self, coordinate: Coordinate) {
        let mut tile = &mut self.tile_array[coordinate.x][coordinate.y];
        if !tile.is_revealed{
            if tile.is_bomb{
                self.lose(&tile);
            }else{
                self.reveal_nearby(&mut tile)
            }
        }
    }

    fn reveal_nearby(&mut self, mut tile: &mut Tile){
        if !tile.is_revealed{
            match tile.number{
                None if tile.is_bomb => {return}
                None => {
                    tile.is_revealed = true;
                    for xd in -1..=1{
                        for yd in -1..=1{
                            // Makes sure that it doesn't go outside of the tile_array index bounds
                            match (tile.coordinates.x+xd, tile.coordinates.y+yd){
                                (x, y) if x < 0 || x >= GAME_COLS || y < 0 || y > GAME_ROWS => {}
                                (x, y) =>{
                                    self.reveal_nearby(&mut self.tile_array[x][y])
                                }
                            }
                        }
                    }
                }
                Some(_) =>{
                    tile.is_revealed = true;
                }
            }
        }



    }

}



fn main() {





}
