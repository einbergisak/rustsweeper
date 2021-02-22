use crate::game::{GameContainer};
use std::{io::stdin, ops::Add};

extern crate piston_window;

mod tests;
mod game;
use piston_window::*;

fn main() {

    let mut start_window: PistonWindow = WindowSettings::new("Enter game settings.", [200,400]).exit_on_esc(true).build().unwrap();
    
    if let Some(event) = start_window.next(){
        start_window.draw_2d(&event, |context, graphics, _device| {
            clear([0.2; 4], graphics);

        });
    } else{
        panic!("Failed to start game. Check that your system is compatible with the Piston library");
    }
    

    let mut main_window: PistonWindow =
        WindowSettings::new("Rustsweeper", [800, 800])
        .exit_on_esc(true).build().unwrap();
    
    while let Some(event) = main_window.next() {
        main_window.draw_2d(&event, |context, graphics, _device| {
            clear([0.2; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
            rectangle([1.0, 1.0, 0.0, 1.0], // red
                      [0.0, 0.0, 50.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}