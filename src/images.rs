use ggez::graphics::{FilterMode, Image};
use ggez::*;

pub struct Images {
    pub grass: Image,
    pub player_front: Image,
    pub player_left: Image,
    pub player_up: Image,
    pub houses: Vec<Image>,
    pub sign: Image,
    pub letter: Image,
    pub arrow: Image,
    pub trees: Image,
    pub progress_bar: Image,
    pub progress_bar_filled: Image,
}

impl Images {
    pub fn new(ctx: &mut Context) -> GameResult<Images> {
        Ok(Images {
            grass: Images::load_image(ctx, "/grass.png")?,
            player_front: Images::load_image(ctx, "/player_front.png")?,
            player_left: Images::load_image(ctx, "/player_left.png")?,
            player_up: Images::load_image(ctx, "/player_up.png")?,
            houses: vec![
                Images::load_image(ctx, "/house_1.png")?,
                Images::load_image(ctx, "/house_2.png")?,
                Images::load_image(ctx, "/house_3.png")?,
                Images::load_image(ctx, "/house_4.png")?,
            ],
            sign: Images::load_image(ctx, "/sign.png")?,
            letter: Images::load_image(ctx, "/letter.png")?,
            arrow: Images::load_image(ctx, "/arrow.png")?,
            trees: Images::load_image(ctx, "/trees.png")?,
            progress_bar: Images::load_image(ctx, "/progress_bar.png")?,
            progress_bar_filled: Images::load_image(ctx, "/progress_bar_filled.png")?,
        })
    }

    fn load_image(ctx: &mut Context, path: &str) -> GameResult<Image> {
        let mut image = Image::new(ctx, path)?;
        image.set_filter(FilterMode::Nearest);
        Ok(image)
    }
}
