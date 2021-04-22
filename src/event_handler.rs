use std::time::Duration;

use ggez::{event, graphics::Text, nalgebra::Vector2};
use ggez::{
    event::EventHandler,
    graphics::{self, DrawParam, Rect},
    input::mouse,
    nalgebra::Point2,
};

use crate::{game::GameContainer, DEFAULT_TILE_SIZE, SPRITESHEET_WIDTH};

const TILE_IMAGE_FRACTION: f32 = DEFAULT_TILE_SIZE / SPRITESHEET_WIDTH;

impl EventHandler for GameContainer {
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        let tile_x = (x / self.scaled_tile_size).floor() as usize;
        let tile_y = (y / self.scaled_tile_size).floor() as usize;
        match button {
            event::MouseButton::Left => {
                if self.tiles_revealed == 0{
                    self.distribute_mines(Some((tile_x, tile_y)))
                }

                if mouse::button_pressed(ctx, event::MouseButton::Right) {
                    // Chord if both the left and the right mouse buttons are pressed.
                    self.chord_at((tile_x, tile_y))
                } else {
                    // Reveal
                    self.reveal_tile_at((tile_x, tile_y));
                }
            }
            ggez::event::MouseButton::Right => {
                if mouse::button_pressed(ctx, event::MouseButton::Left) {
                    // Chord if both the left and the right mouse buttons are pressed.
                    self.chord_at((tile_x, tile_y))
                } else {
                    // Flag
                    self.toggle_flag_at((tile_x, tile_y));
                }
            }
            ggez::event::MouseButton::Middle => {
                // Chord
                self.chord_at((tile_x, tile_y));
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        while ggez::timer::check_update_time(ctx, 60) {}
        Ok(())
    }

    // TODO: Rita endast upp de som det har blivit ändring på.
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(50, 50, 50));
        let mut sprite_batch = self.sprite_batch.clone();
        let tile_array = &*self.tile_array;
        for (x, vec) in tile_array.into_iter().enumerate() {
            for (y, tile) in vec.into_iter().enumerate() {
                // Sets the "source" of the image for each tile, which is a part of the /resources/spritesheet.bmp image which is loaded into the sprite batch.
                let src = if tile.is_flagged {
                    Rect::new(2.0 * TILE_IMAGE_FRACTION, 0.0, TILE_IMAGE_FRACTION, 1.0)
                } else {
                    match (tile.is_revealed, tile.number) {
                        (false, _) => Rect::new(TILE_IMAGE_FRACTION, 0.0, TILE_IMAGE_FRACTION, 1.0),

                        (true, Some(number)) => Rect::new(
                            TILE_IMAGE_FRACTION * (2.0 + number as f32),
                            0.0,
                            TILE_IMAGE_FRACTION,
                            1.0,
                        ),
                        (true, None) => Rect::new(0.0, 0.0, TILE_IMAGE_FRACTION, 1.0),
                    }
                };

                sprite_batch.add(
                    DrawParam::new()
                        .src(src)
                        .scale(Vector2::new(
                            self.scaled_tile_size / DEFAULT_TILE_SIZE,
                            self.scaled_tile_size / DEFAULT_TILE_SIZE,
                        ))
                        .dest(Point2::new(
                            x as f32 * self.scaled_tile_size,
                            y as f32 * self.scaled_tile_size,
                        )),
                );
            }
        }

        graphics::draw(ctx, &sprite_batch, (Point2::<f32>::new(0.0, 0.0),))
            .expect("Something went wrong rendering the game.");


        // let a = ggez::graphics::Mesh::new_circle(
        //     ctx,
        //     DrawMode::Fill(FillOptions::default()),
        //     Point2::new(10.0, 10.0),
        //     5.0,
        //     1.0,
        //     graphics::WHITE,
        // )
        // .unwrap();


            let current_time = self.start_time.unwrap_or(std::time::SystemTime::now()).elapsed().unwrap_or(std::time::Duration::from_secs(0));
            let mut elapsed_time = Text::new(format!("Elapsed time: {}s", current_time.as_secs().to_string()));
            elapsed_time.set_font(
                graphics::Font::default(),
                graphics::Scale { x: 15.0, y: 15.0 },
            );
            graphics::draw(ctx, &elapsed_time, (Point2::<f32>::new(0.0, self.game_rows as f32*self.scaled_tile_size),))?;

        let mut remaining_mines = Text::new(format!("Remaining mines: {}", self.game_mines as isize -self.tiles_flagged));
        remaining_mines.set_font(
            graphics::Font::default(),
            graphics::Scale { x: 15.0, y: 15.0 },
        );
        graphics::draw(ctx, &remaining_mines, (Point2::<f32>::new(150.0, self.game_rows as f32*self.scaled_tile_size),))?;

        graphics::present(ctx)
    }
}
