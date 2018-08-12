extern crate ggez;
extern crate rand;
use ggez::*;

mod images;
mod game;

use images::Images;
use game::Game;

fn main() {
    let cb = ContextBuilder::new("ludum_dare", "Thomas den Hollander")
        .window_setup(conf::WindowSetup::default()
            .title("Ludum Dare")
            .resizable(false)
            .samples(1).unwrap())
        .window_mode(conf::WindowMode::default()
            .dimensions(800, 600));
    let ctx = &mut cb.build().unwrap();
    let images = Images::new(ctx).expect("Could not load images");
    let game = &mut Game::new(ctx, &images).unwrap();
    event::run(ctx, game).unwrap();
}
