extern crate ggez;
extern crate rand;
use ggez::*;

mod game;
mod game_state;
mod images;
mod splash_screen;

use game_state::StateHolder;
use images::Images;

fn main() {
    let cb = ContextBuilder::new("Too Much Post, Out Of Space", "Thomas den Hollander")
        .window_setup(
            conf::WindowSetup::default()
                .title("Too Much Post, Out Of Space")
                .resizable(false)
                .samples(1)
                .unwrap(),
        )
        .window_mode(conf::WindowMode::default().dimensions(800, 600));
    let ctx = &mut cb.build().unwrap();
    let images = Images::new(ctx).expect("Could not load images");
    let mut game_state = StateHolder::startup(ctx, &images).unwrap();
    event::run(ctx, &mut game_state).unwrap();
}
