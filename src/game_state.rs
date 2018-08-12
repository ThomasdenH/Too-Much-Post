use game::Game;
use ggez::event::{EventHandler, Keycode, Mod};
use ggez::*;
use images::Images;
use splash_screen::SplashScreen;

enum GameState<'a> {
    MainGame(Game<'a>),
    SplashScreen(SplashScreen<'a>),
}

pub struct StateHolder<'a> {
    game_state: GameState<'a>,
}

impl<'a> StateHolder<'a> {
    pub fn startup(ctx: &mut Context, images: &'a Images) -> GameResult<StateHolder<'a>> {
        Ok(StateHolder {
            game_state: GameState::SplashScreen(SplashScreen::new(ctx, images)?),
        })
    }
}

impl<'a> EventHandler for StateHolder<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut next_state = None;

        match self.game_state {
            GameState::SplashScreen(ref mut splash_screen) => {
                splash_screen.update(ctx)?;
                if splash_screen.should_start() {
                    next_state = Some(GameState::MainGame(Game::new(ctx, splash_screen.images)?));
                }
            }
            GameState::MainGame(ref mut game) => game.update(ctx)?,
        }

        if let Some(state) = next_state {
            self.game_state = state;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.game_state {
            GameState::SplashScreen(ref mut splash_screen) => {
                splash_screen.draw(ctx)?;
            }
            GameState::MainGame(ref mut game) => game.draw(ctx)?,
        }
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        match self.game_state {
            GameState::SplashScreen(ref mut splash_screen) => {
                splash_screen.key_down_event(ctx, keycode, keymod, repeat)
            }
            GameState::MainGame(ref mut game) => game.key_down_event(ctx, keycode, keymod, repeat),
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        match self.game_state {
            GameState::SplashScreen(ref mut splash_screen) => {
                splash_screen.key_up_event(ctx, keycode, keymod, repeat)
            }
            GameState::MainGame(ref mut game) => game.key_up_event(ctx, keycode, keymod, repeat),
        }
    }
}
