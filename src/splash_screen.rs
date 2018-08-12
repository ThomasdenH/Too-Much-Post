use ggez::event::EventHandler;
use ggez::graphics::DrawParam;
use ggez::graphics::Drawable;
use ggez::graphics::FilterMode;
use ggez::graphics::Point2;
use ggez::graphics::{ Text, Rect };
use ggez::*;
use images::Images;
use ggez::event::{ Keycode, Mod };

pub struct SplashScreen<'a> {
    duration: f64,
    pub images: &'a Images,
    main_text: Text,
    sub_text: Text,

    pick_up_letters_text: Text,
    deliver_text: Text,
    hold_four_text: Text,
    game_over_text: Text,
    spacebar_text: Text,

    spacebar_pressed: bool
}

impl<'a> SplashScreen<'a> {
    pub fn new(ctx: &mut Context, images: &'a Images) -> GameResult<SplashScreen<'a>> {
        let mut main_text = Text::new(
            ctx,
            &"Too Much Post, Out Of Space",
            &graphics::Font::default_font()?,
        )?;
        main_text.set_filter(FilterMode::Nearest);

        let mut sub_text = Text::new(
            ctx,
            &"Thomas den Hollander (Ludum Dare 42)",
            &graphics::Font::default_font()?,
        )?;
        sub_text.set_filter(FilterMode::Nearest);

        let mut pick_up_letters_text = Text::new(
            ctx,
            &"Pick up letters...",
            &graphics::Font::default_font()?,
        )?;
        pick_up_letters_text.set_filter(FilterMode::Nearest);

        let mut deliver_text = Text::new(
            ctx,
            &"...and deliver them to the correct houses.",
            &graphics::Font::default_font()?,
        )?;
        deliver_text.set_filter(FilterMode::Nearest);

        let mut game_over_text = Text::new(
            ctx,
            &"The game ends when the ground is full...",
            &graphics::Font::default_font()?,
        )?;
        game_over_text.set_filter(FilterMode::Nearest);

        let mut hold_four_text = Text::new(
            ctx,
            &"You'll drop them if you carry too many!",
            &graphics::Font::default_font()?,
        )?;
        hold_four_text.set_filter(FilterMode::Nearest);

        let mut spacebar_text = Text::new(
            ctx,
            &"Press Space to continue.",
            &graphics::Font::default_font()?,
        )?;
        spacebar_text.set_filter(FilterMode::Nearest);

        Ok(SplashScreen {
            duration: 0.0,
            images,
            main_text,
            sub_text,

            pick_up_letters_text,
            deliver_text,
            hold_four_text,
            game_over_text,
            spacebar_text,

            spacebar_pressed: false
        })
    }

    pub fn should_start(&self) -> bool {
        self.spacebar_pressed
    }
}

impl<'a> EventHandler for SplashScreen<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.duration += timer::duration_to_f64(timer::get_delta(ctx));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        if self.duration < 3.0 {
            let dy =
                (timer::duration_to_f64(timer::get_time_since_start(ctx)) * 5.0).sin() as f32 * 20.0;
            self.images.houses[2].draw_ex(
                ctx,
                DrawParam {
                    dest: Point2::new(100.0, 300.0 - 64.0 + dy),
                    scale: Point2::new(2.0, 2.0),
                    ..Default::default()
                },
            )?;

            self.main_text.draw_ex(
                ctx,
                DrawParam {
                    dest: Point2::new(250.0, 270.0),
                    scale: Point2::new(2.0, 2.0),
                    ..Default::default()
                },
            )?;
            self.sub_text.draw_ex(
                ctx,
                DrawParam {
                    dest: Point2::new(270.0, 320.0),
                    ..Default::default()
                },
            )?;
        } else {
            self.pick_up_letters_text.draw(ctx, Point2::new(340.0, 85.0), 0.0)?;
            self.images.letter.draw_ex(ctx, DrawParam {
                dest: Point2::new(200.0, 60.0),
                scale: Point2::new(2.0, 2.0),
                ..Default::default()
            })?;
            self.deliver_text.draw(ctx, Point2::new(200.0, 200.0), 0.0)?;
            self.images.houses[3].draw_ex(ctx, DrawParam {
                dest: Point2::new(550.0, 145.0),
                scale: Point2::new(2.0, 2.0),
                ..Default::default()
            })?;
            self.hold_four_text.draw(ctx, Point2::new(340.0, 315.0), 0.0)?;
            self.images.player_front.draw_ex(ctx, DrawParam {
                dest: Point2::new(200.0, 280.0),
                scale: Point2::new(2.5, 2.5),
                ..Default::default()
            })?;
            self.game_over_text.draw(ctx, Point2::new(200.0, 450.0), 0.0)?;

            // Draw progress bar
            let progress_bar_dest = Point2::new(550.0, 450.0);
            self.images.progress_bar.draw_ex(
                ctx,
                DrawParam {
                    dest: progress_bar_dest,
                    rotation: 0.0,
                    offset: Point2::new(0.0, 0.5),
                    shear: Point2::new(0.0, 0.0),
                    ..Default::default()
                },
            )?;
            let time = timer::duration_to_f64(timer::get_time_since_start(ctx));
            let fraction = (time - time.floor()) as f32;
            self.images.progress_bar_filled.draw_ex(
                ctx,
                DrawParam {
                    src: Rect {
                        x: 0.0,
                        y: 0.0,
                        w: fraction,
                        h: 1.0,
                    },
                    dest: progress_bar_dest,
                    rotation: 0.0,
                    offset: Point2::new(0.0, 0.5),
                    shear: Point2::new(0.0, 0.0),
                    ..Default::default()
                },
            )?;
            self.spacebar_text.draw_ex(
                ctx,
                DrawParam {
                    dest: Point2::new(400.0, 550.0),
                    rotation: 0.0,
                    offset: Point2::new(0.5, 0.5),
                    ..Default::default()
                },
            )?;
        }
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        if keycode == Keycode::Space {
            self.spacebar_pressed = true;
        }
    }
}
