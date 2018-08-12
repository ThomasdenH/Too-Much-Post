use ggez::event::{EventHandler, Keycode, Mod};
use ggez::graphics::{DrawParam, Drawable, FilterMode, Point2, Rect, Text, Vector2};
use ggez::*;
use std::collections::VecDeque;

use images::Images;

use std::f32::consts::PI;

use rand::{thread_rng, Rng};

const LEVEL_WIDTH: u32 = 30;
const LEVEL_HEIGHT: u32 = 20;

const TILES_ON_WIDTH: f32 = 15.0;

const SPAWN_TIME: f32 = 30.0;

const HOUSE_COUNT: u32 = 6;

const MAX_HOLDING: usize = 4;

const PLAYER_SPEED: f32 = 7.0;
const MAX_LETTERS_ON_GROUND: u32 = 10;

#[derive(Clone, Debug)]
struct House {
    position: Point2,
    number: u32,
    text: Text,
    resource_type: usize,
}

impl House {
    fn new(
        ctx: &mut Context,
        position: Point2,
        number: u32,
        image_count: usize,
    ) -> GameResult<House> {
        let mut text = Text::new(
            ctx,
            &format!("{}", number),
            &graphics::Font::default_font()?,
        )?;
        text.set_filter(FilterMode::Nearest);
        Ok(House {
            position,
            number,
            text,
            resource_type: thread_rng().gen_range(0, image_count),
        })
    }

    fn draw(&self, ctx: &mut Context, game: &Game, offset: Vector2) -> GameResult<()> {
        let scale = Game::tile_size(ctx) / 32.0;
        let dest = Game::float_coord_to_screen(ctx, self.position - game.player.coords);
        graphics::draw_ex(
            ctx,
            &game.images.houses[self.resource_type],
            DrawParam {
                dest: dest + offset,
                rotation: 0.0,
                offset: Point2::new(0.5, 0.5),
                shear: Point2::new(0.0, 0.0),
                scale: scale * 2.0,
                ..Default::default()
            },
        )?;

        graphics::draw_ex(
            ctx,
            &game.images.sign,
            DrawParam {
                dest: dest + offset,
                rotation: 0.0,
                offset: Point2::new(0.0, -0.2),
                shear: Point2::new(0.0, 0.0),
                scale,
                ..Default::default()
            },
        )?;
        let number_dest = Game::float_coord_to_screen(
            ctx,
            self.position - game.player.coords + Vector2::new(1.0, 1.3),
        );
        self.text.draw_ex(
            ctx,
            DrawParam {
                dest: number_dest + offset,
                rotation: 0.0,
                offset: Point2::new(0.5, 0.5),
                shear: Point2::new(0.0, 0.0),
                scale,
                ..Default::default()
            },
        )?;
        Ok(())
    }

    fn intersects(&self, other: &House) -> bool {
        let diff = self.position - other.position;
        diff[0].abs() < 4.0 && diff[1].abs() < 4.0
    }

    fn player_intersection(&self, ctx: &mut Context, game: &Game) -> bool {
        let screen_coords = graphics::get_screen_coordinates(ctx);
        let w = screen_coords.w;
        let h = screen_coords.h;
        let player_pos = Point2::new(w / 2.0, h / 2.0);

        let dest = Game::float_coord_to_screen(ctx, self.position - game.player.coords);

        let dist = dest - player_pos;

        (dist[0]).abs() < 64.0 && (dist[1]).abs() < 64.0
    }
}

#[derive(Clone, Debug)]
struct Letter {
    number: u32,
    text: Text,
    position: Point2,
    dropped_time: f64,
}

impl Letter {
    fn new(ctx: &mut Context, position: Point2, number: u32) -> GameResult<Letter> {
        let text = Text::new(
            ctx,
            &format!("{}", number),
            &graphics::Font::default_font()?,
        )?;
        Ok(Letter {
            number,
            text,
            position,
            dropped_time: 0.0,
        })
    }

    fn player_intersection(&self, ctx: &mut Context, game: &Game) -> bool {
        let screen_coords = graphics::get_screen_coordinates(ctx);
        let w = screen_coords.w;
        let h = screen_coords.h;
        let player_pos = Point2::new(w / 2.0, h / 2.0);

        let dest = Game::float_coord_to_screen(ctx, self.position - game.player.coords);

        let dist = dest - player_pos;

        (dist[0]).abs() < 48.0
            && (dist[1]).abs() < 48.0
            && timer::duration_to_f64(timer::get_time_since_start(ctx)) > self.dropped_time
    }

    fn draw(&self, ctx: &mut Context, game: &Game, offset: Vector2) -> GameResult<()> {
        let scale = Game::tile_size(ctx) / 32.0;
        let dest = Game::float_coord_to_screen(ctx, self.position - game.player.coords);

        let time = timer::duration_to_f64(timer::get_time_since_start(ctx));

        game.images.letter.draw_ex(
            ctx,
            DrawParam {
                dest: dest + offset,
                rotation: 0.0,
                offset: Point2::new(0.5, 0.5),
                shear: Point2::new(0.0, 0.0),
                scale: scale * 1.5,
                color: if self.dropped_time > time {
                    Some(graphics::Color::new(0.7, 0.7, 0.7, 1.0))
                } else {
                    None
                },
                ..Default::default()
            },
        )?;
        self.text.draw_ex(
            ctx,
            DrawParam {
                dest: dest + offset,
                rotation: 0.0,
                offset: Point2::new(0.5, 0.5),
                shear: Point2::new(0.0, 0.0),
                scale,
                color: Some(graphics::Color::new(0.0, 0.0, 0.0, 1.0)),
                ..Default::default()
            },
        )?;
        Ok(())
    }

    fn draw_in_hand(&self, ctx: &mut Context, game: &Game, i: usize) -> GameResult<()> {
        let scale = Game::tile_size(ctx) / 32.0;
        let dest = Game::float_coord_to_screen(
            ctx,
            Point2::new(scale[0] * 0.5, scale[0] * 0.5 + scale[1] * 0.6 * i as f32),
        );

        game.images.letter.draw_ex(
            ctx,
            DrawParam {
                dest,
                rotation: 0.0,
                offset: Point2::new(0.5, 0.5),
                shear: Point2::new(0.0, 0.0),
                scale: scale * 1.5,
                ..Default::default()
            },
        )?;
        self.text.draw_ex(
            ctx,
            DrawParam {
                dest,
                rotation: 0.0,
                offset: Point2::new(0.5, 0.5),
                shear: Point2::new(0.0, 0.0),
                scale,
                color: Some(graphics::Color::new(0.0, 0.0, 0.0, 1.0)),
                ..Default::default()
            },
        )?;
        Ok(())
    }
}

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

pub struct Game<'a> {
    player: Point2,
    images: &'a Images,

    right_pressed: bool,
    left_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,

    player_direction: Direction,
    player_running: bool,

    houses: Vec<House>,
    letters: Vec<Letter>,
    holding_letters: VecDeque<Letter>,

    time_since_last_letter: f32,

    score: u32,
    score_text: Text,

    game_over: bool,

    letter_animation: VecDeque<f32>,

    music: audio::Source,

    sound_pickup: audio::Source,
    sound_drop: audio::Source,
    sound_drop_bad: audio::Source,
    letter_spawn_time: f32,

    game_over_text: Text
}

impl<'a> Game<'a> {
    pub fn new(ctx: &mut Context, images: &'a Images) -> GameResult<Game<'a>> {
        let houses = Game::generate_houses(ctx, images.houses.len())?;

        let mut game_over_text = Text::new(
            ctx,
            &"Game over - press spacebar to restart",
            &graphics::Font::default_font()?,
        )?;
        game_over_text.set_filter(FilterMode::Nearest);

        Ok(Game {
            player: Point2::new(10.0, 10.0),
            images,

            right_pressed: false,
            left_pressed: false,
            up_pressed: false,
            down_pressed: false,

            player_direction: Direction::Down,
            player_running: false,
            houses,
            letters: Vec::new(),
            holding_letters: VecDeque::new(),

            time_since_last_letter: 0.0,

            score: 0,
            score_text: Game::get_score_text(0, ctx)?,

            game_over: false,

            letter_animation: VecDeque::new(),

            music: audio::Source::new(ctx, "/music.wav")?,

            sound_pickup: audio::Source::new(ctx, "/pickup.wav")?,
            sound_drop: audio::Source::new(ctx, "/drop.wav")?,
            sound_drop_bad: audio::Source::new(ctx, "/drop_bad.wav")?,
            letter_spawn_time: SPAWN_TIME,
            game_over_text
        })
    }

    fn generate_houses(ctx: &mut Context, resource_count: usize) -> GameResult<Vec<House>> {
        let mut houses = Vec::new();
        for house_number in 0..HOUSE_COUNT {
            loop {
                let position = Point2::new(
                    thread_rng().gen_range(3.0, LEVEL_WIDTH as f32 - 3.0),
                    thread_rng().gen_range(3.0, LEVEL_HEIGHT as f32 - 3.0),
                );
                let new_house = House::new(ctx, position, 1 + house_number, resource_count)?;
                if houses
                    .iter()
                    .all(|house: &House| !house.intersects(&new_house))
                {
                    houses.push(new_house);
                    break;
                }
            }
        }
        Ok(houses)
    }

    fn restart(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.player = Point2::new(10.0, 10.0);
        self.houses = Game::generate_houses(ctx, self.images.houses.len())?;
        self.letters = Vec::new();
        self.holding_letters = VecDeque::new();
        self.time_since_last_letter = 0.0;
        self.score = 0;
        self.score_text = Game::get_score_text(0, ctx)?;
        self.game_over = false;
        self.letter_animation = VecDeque::new();
        self.letter_spawn_time = SPAWN_TIME;
        Ok(())
    }

    fn get_score_text(score: u32, ctx: &mut Context) -> GameResult<Text> {
        let mut text = Text::new(
            ctx,
            &format!("Score: {}", score),
            &graphics::Font::default_font()?,
        )?;

        text.set_filter(FilterMode::Nearest);
        Ok(text)
    }

    fn float_coord_to_screen(ctx: &Context, point: Point2) -> Point2 {
        let width = graphics::get_screen_coordinates(ctx).w;
        Point2::new(
            point[0] / TILES_ON_WIDTH * width,
            point[1] / TILES_ON_WIDTH * width,
        )
    }

    fn screen_coord_to_float(ctx: &Context, point: Point2) -> Point2 {
        let width = graphics::get_screen_coordinates(ctx).w;
        Point2::new(
            point[0] * TILES_ON_WIDTH / width,
            point[1] * TILES_ON_WIDTH / width,
        )
    }

    fn tile_size(ctx: &Context) -> Point2 {
        let width = graphics::get_screen_coordinates(ctx).w;
        Point2::new(width / TILES_ON_WIDTH, width / TILES_ON_WIDTH)
    }

    fn calculate_player_direction(&mut self) {
        self.player_running = true;
        if self.left_pressed && !self.right_pressed {
            // Left
            if self.up_pressed && !self.down_pressed {
                self.player_direction = Direction::UpLeft;
            } else if self.down_pressed && !self.up_pressed {
                self.player_direction = Direction::DownLeft;
            } else {
                self.player_direction = Direction::Left;
            }
        } else if self.right_pressed && !self.left_pressed {
            // Right
            if self.up_pressed && !self.down_pressed {
                self.player_direction = Direction::UpRight;
            } else if self.down_pressed && !self.up_pressed {
                self.player_direction = Direction::DownRight;
            } else {
                self.player_direction = Direction::Right;
            }
        } else {
            if self.up_pressed && !self.down_pressed {
                self.player_direction = Direction::Up;
            } else if self.down_pressed && !self.up_pressed {
                self.player_direction = Direction::Down;
            } else {
                self.player_running = false;
            }
        }
    }

    fn draw_trees(&mut self, ctx: &mut Context, offset: Vector2) -> GameResult<()> {
        let scale = Game::tile_size(ctx) / 32.0;
        for i in 0..LEVEL_HEIGHT - 8 {
            let pos = Point2::new(-0.8 - self.player[0], scale[0] * i as f32 - self.player[1]);
            let dest = Game::float_coord_to_screen(ctx, pos);
            self.images.trees.draw_ex(
                ctx,
                DrawParam {
                    dest: dest + offset,
                    rotation: 0.0,
                    offset: Point2::new(0.5, 0.5),
                    shear: Point2::new(0.0, 0.0),
                    scale,
                    ..Default::default()
                },
            )?;
        }
        for i in 0..LEVEL_HEIGHT - 8 {
            let pos = Point2::new(
                LEVEL_WIDTH as f32 - 0.2 - self.player[0],
                scale[0] * i as f32 - self.player[1],
            );
            let dest = Game::float_coord_to_screen(ctx, pos);
            self.images.trees.draw_ex(
                ctx,
                DrawParam {
                    dest: dest + offset,
                    rotation: PI,
                    offset: Point2::new(0.5, 0.5),
                    shear: Point2::new(0.0, 0.0),
                    scale,
                    ..Default::default()
                },
            )?;
        }
        for i in 0..LEVEL_WIDTH - 12 {
            let pos = Point2::new(scale[0] * i as f32 - self.player[0], -0.8 - self.player[1]);
            let dest = Game::float_coord_to_screen(ctx, pos);
            self.images.trees.draw_ex(
                ctx,
                DrawParam {
                    dest: dest + offset,
                    rotation: 0.5 * PI,
                    offset: Point2::new(0.5, 0.5),
                    shear: Point2::new(0.0, 0.0),
                    scale,
                    ..Default::default()
                },
            )?;
        }
        for i in 0..LEVEL_WIDTH - 11 {
            let pos = Point2::new(
                scale[0] * i as f32 - self.player[0],
                LEVEL_HEIGHT as f32 - 0.2 - self.player[1],
            );
            let dest = Game::float_coord_to_screen(ctx, pos);
            self.images.trees.draw_ex(
                ctx,
                DrawParam {
                    dest: dest + offset,
                    rotation: 1.5 * PI,
                    offset: Point2::new(0.5, 0.5),
                    shear: Point2::new(0.0, 0.0),
                    scale,
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }
}

impl<'a> EventHandler for Game<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.music.playing() {
            self.music.repeat();
            self.music.play()?;
        }

        if self.game_over {

        } else {
            let delta = timer::duration_to_f64(timer::get_delta(ctx));
            let d = delta as f32 * PLAYER_SPEED;
            if self.player_running {
                self.player += match self.player_direction {
                    Direction::Up => Vector2::new(0.0, -d),
                    Direction::Down => Vector2::new(0.0, d),
                    Direction::Left => Vector2::new(-d, 0.0),
                    Direction::Right => Vector2::new(d, 0.0),
                    Direction::UpLeft => Vector2::new(-0.70 * d, -0.70 * d),
                    Direction::DownLeft => Vector2::new(-0.70 * d, 0.70 * d),
                    Direction::UpRight => Vector2::new(0.70 * d, -0.70 * d),
                    Direction::DownRight => Vector2::new(0.70 * d, 0.70 * d),
                };

                let screen_coords = graphics::get_screen_coordinates(ctx);
                let w = screen_coords.w;
                let h = screen_coords.h;
                let player_pos = Point2::new(w / 2.0, h / 2.0);
                let diff = Game::screen_coord_to_float(ctx, player_pos);
                if self.player[0] + diff[0] < 0.0 {
                    self.player[0] = -diff[0];
                } else if self.player[0] + diff[0] > LEVEL_WIDTH as f32 - 1.0 {
                    self.player[0] = LEVEL_WIDTH as f32 - diff[0] - 1.0;
                }

                if self.player[1] + diff[1] < 0.0 {
                    self.player[1] = -diff[1];
                } else if self.player[1] + diff[1] > LEVEL_HEIGHT as f32 - 1.0 {
                    self.player[1] = LEVEL_HEIGHT as f32 - diff[1] - 1.0;
                }
            }

            self.time_since_last_letter += d;
            while self.time_since_last_letter > self.letter_spawn_time {
                self.time_since_last_letter -= self.letter_spawn_time;
                self.letter_spawn_time *= 0.95;
                let position = Point2::new(
                    thread_rng().gen_range(2.0, LEVEL_WIDTH as f32 - 2.0),
                    thread_rng().gen_range(2.0, LEVEL_HEIGHT as f32 - 2.0),
                );
                let number = thread_rng().gen_range(1, HOUSE_COUNT + 1);
                self.letters.push(Letter::new(ctx, position, number)?);
            }

            // Pick up letters
            let mut not_picked_up = Vec::new();
            let mut any_picked_up = false;
            let mut any_dropped = false;
            while self.letters.len() > 0 {
                let letter = self.letters.pop().unwrap();
                if letter.player_intersection(ctx, &self) {
                    self.holding_letters.push_front(letter);
                    any_picked_up = true;
                    if self.holding_letters.len() > MAX_HOLDING {
                        let mut dropped_letter = self.holding_letters.pop_back().unwrap();
                        dropped_letter.dropped_time =
                            timer::duration_to_f64(timer::get_time_since_start(ctx)) + 2.0;
                        let screen_coords = graphics::get_screen_coordinates(ctx);
                        let w = screen_coords.w;
                        let h = screen_coords.h;
                        let player_pos = Vector2::new(w / 2.0, h / 2.0);
                        dropped_letter.position = Game::screen_coord_to_float(
                            ctx,
                            Game::float_coord_to_screen(ctx, self.player) + player_pos,
                        );
                        not_picked_up.push(dropped_letter);
                        any_dropped = true;
                    }
                } else {
                    not_picked_up.push(letter);
                }
            }
            self.letters = not_picked_up;
            if any_dropped {
                self.sound_drop_bad.play()?;
            } else if any_picked_up {
                self.sound_pickup.play()?;
            }

            // Drop off letters
            let mut updated_score = false;
            for house in &self.houses {
                if house.player_intersection(ctx, &self) {
                    while self.holding_letters.len() > 0
                        && self.holding_letters[0].number == house.number
                    {
                        self.holding_letters.pop_front();
                        self.score += 1;
                        updated_score = true;
                    }
                }
            }
            if updated_score {
                self.score_text = Game::get_score_text(self.score, ctx)?;
                self.letter_animation.push_front(0.0);
                self.sound_drop.play()?;
            }

            if self.letters.len() >= MAX_LETTERS_ON_GROUND as usize {
                self.game_over = true;
            }

            for animated_letter in &mut self.letter_animation {
                *animated_letter += 5.0 * delta as f32;
            }
            while self.letter_animation.back().is_some()
                && *self.letter_animation.back().unwrap() > 1.0
            {
                self.letter_animation.pop_back();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let screen_coords = graphics::get_screen_coordinates(ctx);
        let w = screen_coords.w;
        let h = screen_coords.h;

        let tiles_on_height = TILES_ON_WIDTH / w * h;

        let offset_x = if self.player[0] < -1.0 {
            self.player[0] + 1.0
        } else if self.player[0] > LEVEL_WIDTH as f32 - TILES_ON_WIDTH {
            self.player[0] - (LEVEL_WIDTH as f32 - TILES_ON_WIDTH)
        } else {
            0.0
        };

        let offset_y = if self.player[1] < -1.0 {
            self.player[1] + 1.0
        } else if self.player[1] > LEVEL_HEIGHT as f32 - tiles_on_height {
            self.player[1] - (LEVEL_HEIGHT as f32 - tiles_on_height)
        } else {
            0.0
        };

        let offset: Vector2 =
            Game::float_coord_to_screen(ctx, Point2::new(offset_x, offset_y)).coords;

        let scale = Game::tile_size(ctx) / 32.0;

        for x in 0..LEVEL_WIDTH / 10 {
            for y in 0..LEVEL_HEIGHT / 10 {
                let pos = Point2::new(
                    x as f32 * 10.0 - self.player[0],
                    y as f32 * 10.0 - self.player[1],
                );
                let dest = Game::float_coord_to_screen(ctx, pos);
                graphics::draw_ex(
                    ctx,
                    &self.images.grass,
                    DrawParam {
                        dest: dest + offset,
                        rotation: 0.0,
                        offset: Point2::new(0.0, 0.0),
                        shear: Point2::new(0.0, 0.0),
                        scale,
                        ..Default::default()
                    },
                )?;
            }
        }

        // Draw houses
        for house in &self.houses {
            house.draw(ctx, &self, offset)?;
        }

        // Draw letters
        for letter in &self.letters {
            letter.draw(ctx, &self, offset)?;
        }

        let player_pos = Point2::new(w / 2.0, h / 2.0);
        let mut player_scale = scale.clone();
        let image = match self.player_direction {
            Direction::Left | Direction::DownLeft => &self.images.player_left,
            Direction::Right | Direction::DownRight => {
                player_scale[0] *= -1.0;
                &self.images.player_left
            }
            Direction::Up | Direction::UpLeft | Direction::UpRight => &self.images.player_up,
            _ => &self.images.player_front,
        };
        graphics::draw_ex(
            ctx,
            image,
            DrawParam {
                dest: player_pos + offset,
                rotation: 0.0,
                offset: Point2::new(0.5, 0.5),
                shear: Point2::new(0.0, 0.0),
                scale: player_scale,
                ..Default::default()
            },
        )?;

        for animated_letter in &self.letter_animation {
            self.images.letter.draw_ex(
                ctx,
                DrawParam {
                    dest: player_pos + Vector2::new(0.0, -20.0 + animated_letter * -32.0) + offset,
                    rotation: 0.0,
                    offset: Point2::new(0.5, 0.5),
                    ..Default::default()
                },
            )?;
        }

        self.draw_trees(ctx, offset)?;

        // Draw picked up letters
        for (i, letter) in self.holding_letters.iter().enumerate() {
            letter.draw_in_hand(ctx, &self, i)?;
        }

        self.images.arrow.draw_ex(
            ctx,
            DrawParam {
                dest: Point2::new(scale[0] * 2.0 * 32.0, scale[0] * 1.0 * 32.0),
                rotation: 0.0,
                offset: Point2::new(0.5, 0.5),
                shear: Point2::new(0.0, 0.0),
                scale,
                ..Default::default()
            },
        )?;

        self.score_text
            .draw(ctx, Point2::new(w - 100.0, 10.0), 0.0)?;

        // Draw progress bar
        self.images.progress_bar.draw_ex(
            ctx,
            DrawParam {
                dest: Point2::new(w - 100.0, scale[0] * 1.0 * 32.0),
                rotation: 0.0,
                offset: Point2::new(0.0, 0.5),
                shear: Point2::new(0.0, 0.0),
                scale: Point2::new(scale[0] * 0.75, scale[1] / 2.0),
                ..Default::default()
            },
        )?;
        let fraction = ((self.letters.len() as f32
            + self.time_since_last_letter / self.letter_spawn_time)/
            MAX_LETTERS_ON_GROUND as f32).min(1.0);
        self.images.progress_bar_filled.draw_ex(
            ctx,
            DrawParam {
                src: Rect {
                    x: 0.0,
                    y: 0.0,
                    w: fraction,
                    h: 1.0,
                },
                dest: Point2::new(w - 100.0, scale[0] * 1.0 * 32.0),
                rotation: 0.0,
                offset: Point2::new(0.0, 0.5),
                shear: Point2::new(0.0, 0.0),
                scale: Point2::new(scale[0] * 0.75, scale[1] / 2.0),
                ..Default::default()
            },
        )?;

        if self.game_over {
            self.game_over_text.draw_ex(ctx,
                DrawParam {
                    dest: Point2::new(400.0, 300.0),
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(2.0, 2.0),
                    ..Default::default()
                }
            )?;
        }

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        if self.game_over {
            if keycode == Keycode::Space {
                self.restart(ctx).unwrap();
            }
        } else {
            match keycode {
                Keycode::Left | Keycode::A => self.left_pressed = true,
                Keycode::Right | Keycode::D => self.right_pressed = true,
                Keycode::Up | Keycode::W => self.up_pressed = true,
                Keycode::Down | Keycode::S => self.down_pressed = true,
                _ => {}
            }
            self.calculate_player_direction();
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool)
    {
        if !self.game_over {
            match keycode {
                Keycode::Left | Keycode::A => self.left_pressed = false,
                Keycode::Right | Keycode::D => self.right_pressed = false,
                Keycode::Up | Keycode::W => self.up_pressed = false,
                Keycode::Down | Keycode::S => self.down_pressed = false,
                _ => {}
            }
            self.calculate_player_direction();
        }
    }
}
