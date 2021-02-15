use crate::game_container::{GameContainer};
use crate::tile::{Tile, Coordinate};
use std::ops::Add;

mod tests;
mod game_container;
mod tile;




fn print_game(game: & GameContainer){
    for col in game.tile_array.iter(){
        let mut string = String::new();
        for tile in col{
            if tile.is_revealed{
                // println!("Found revealed tile! : {:?}", tile);
                match tile.number{
                    None => {string.push('\u{25A1}')}
                    Some(n) => {string.push_str(format!("{}",n).as_str())}
                }
            }else{
                string.push('\u{25A9}');
            }
            string.push_str("  ");
        }
        println!("{}",string);
    }
    println!("-------------------------------------")
}



fn main() {
    let mut game = GameContainer::new("testt");
    print_game(&game);
    game.reveal_tile_at(&Coordinate{x:0, y: 0});
    game.reveal_tile_at(&Coordinate{x:15, y: 15});
    print_game(&game);
}
