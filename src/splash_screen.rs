use ggez::event::EventHandler;
use ggez::graphics::DrawParam;
use ggez::graphics::Drawable;
use ggez::graphics::FilterMode;
use ggez::graphics::Point2;
use ggez::graphics::Text;
use ggez::*;
use images::Images;

pub struct SplashScreen<'a> {
    duration: f64,
    pub images: &'a Images,
    main_text: Text,
    sub_text: Text,
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

        Ok(SplashScreen {
            duration: 0.0,
            images,
            main_text,
            sub_text,
        })
    }

    pub fn should_start(&self) -> bool {
        self.duration > 3.0
    }
}

impl<'a> EventHandler for SplashScreen<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.duration += timer::duration_to_f64(timer::get_delta(ctx));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
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

        graphics::present(ctx);
        Ok(())
    }
}
