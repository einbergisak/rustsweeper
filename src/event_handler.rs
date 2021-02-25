use ggez::nalgebra::Vector2;
use ggez::{
    event::EventHandler,
    graphics::{self, Color, DrawParam, Rect},
    nalgebra::Point2,
};

use crate::game::GameContainer;
const DEFAULT_TILE_SIZE: f32 = 40.0;

impl EventHandler for GameContainer {
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, Color::from_rgb(50, 50, 50));

        // Scales the size of the tiles depending on game settings, this in order to make sure that the game fits on the screen.
        let scaled_tile_size = f32::min(DEFAULT_TILE_SIZE, 1800.0 / self.game_cols as f32)
            .min(1000.0 / self.game_rows as f32);
        let mut sprite_batch = self.sprite_batch.clone();
        let mut dp_vec: Vec<DrawParam> = Vec::new();
        for (x, vec) in self.get_tiles().into_iter().enumerate() {
            for (y, tile) in vec.into_iter().enumerate() {
                // Sets the "source" of the image for each tile, which is a part of the /resources/spritesheet.bmp image which is loaded into the sprite batch.
                let src = if tile.is_flagged {
                    Rect::new(
                        2.0 * DEFAULT_TILE_SIZE,
                        0.0,
                        DEFAULT_TILE_SIZE,
                        DEFAULT_TILE_SIZE,
                    )
                } else {
                    match (tile.is_revealed, tile.number) {
                        (false, _) => {
                            Rect::new(DEFAULT_TILE_SIZE, 0.0, DEFAULT_TILE_SIZE, DEFAULT_TILE_SIZE)
                        }
                        (true, Some(number)) => Rect::new(
                            DEFAULT_TILE_SIZE * (2.0 + number as f32),
                            0.0,
                            DEFAULT_TILE_SIZE,
                            DEFAULT_TILE_SIZE,
                        ),
                        (true, None) => Rect::new(0.0, 0.0, DEFAULT_TILE_SIZE, DEFAULT_TILE_SIZE),
                    }
                };

                dp_vec.push(
                    DrawParam::new()
                        .src(src)
                        .scale(Vector2::new(
                            scaled_tile_size / DEFAULT_TILE_SIZE,
                            scaled_tile_size / DEFAULT_TILE_SIZE,
                        ))
                        .dest(Point2::new(
                            x as f32 * scaled_tile_size,
                            y as f32 * scaled_tile_size,
                        )),
                );
            }
        }
        for dp in dp_vec {
            sprite_batch.add(dp);
        }

        graphics::draw(ctx, &sprite_batch, (Point2::<f32>::new(0.0, 0.0),))
            .expect("Something went wrong rendering the game.");

        graphics::present(ctx)
    }
}
