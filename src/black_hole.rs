use graphics::{Context, ImageSize};
use opengl_graphics::{GlGraphics, Texture};
use piston_window::TextureSettings;
use rust_embed::EmbeddedFile;
use sprite::Sprite;
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use rand::Rng;
use sdl2::rect::Rect;
use crate::game::Game;
use crate::game_sprite::GameSprite;

const ROTATION_UPDATE_MILLIS: Duration = Duration::from_millis(250);

#[derive(Clone, Copy, PartialEq)]
pub enum BlackHoleState {
    Covered,
    Open
}

pub struct BlackHole {
    last_update: SystemTime,
    sprite: GameSprite,
    state: BlackHoleState,
}

impl BlackHole {
    pub fn new(r: &Rect) -> Self {
        Self {
            last_update: SystemTime::now(),
            sprite: GameSprite::from_rect(r),
            state: BlackHoleState::Open
        }
    }

    pub fn covered(&mut self) {
        self.state = BlackHoleState::Covered;
    }

    pub fn get_sprite(&self) -> GameSprite {
        self.sprite
    }

    pub fn get_state(&self) -> BlackHoleState {
        self.state
    }
}

pub struct BlackHoles {
    black_holes: Vec<BlackHole>,
    sprite: Sprite<Texture>,
    sprite_height: u32,
    sprite_width: u32
}

impl BlackHoles {
    pub fn new(sprite_file: &EmbeddedFile) -> Self {
        let image = image::load_from_memory(sprite_file.data.as_ref()).unwrap();
        let texture = Texture::from_image(image.as_rgba8().unwrap(), &TextureSettings::new());
        let size = texture.get_size();

        Self {
            black_holes: Vec::new(),
            sprite: Sprite::from_texture(Rc::new(texture)),
            sprite_height: size.1,
            sprite_width: size.0
        }
    }

    pub fn get_black_holes(&mut self) -> &mut Vec<BlackHole> {
        &mut self.black_holes
    }

    pub fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        for black_hole in self.black_holes.iter() {
            self.sprite.set_position(black_hole.sprite.x, black_hole.sprite.y);
            self.sprite.set_rotation(black_hole.sprite.degrees);
            self.sprite.draw(ctx.transform, gl);
        }
    }

    fn new_rect(&self, sprite_width: u32, sprite_height: u32, window_width: f64, window_height: f64) -> Rect {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, window_width as u32);
        let y = rng.gen_range(0, window_height as u32);

        Rect::new(x as i32, y as i32, sprite_width, sprite_height)
    }

    fn set_black_holes(
        &mut self,
        player_width: u32,
        player_height: u32,
        screen_width: f64,
        screen_height: f64,
        black_hole_count: u32
    ) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, screen_width as u32);
        let y = rng.gen_range(0, screen_height as u32);

        let no_spawn_rect = Rect::new(x as i32, y as i32, player_width, player_height);
        for _ in 0..black_hole_count {
            let mut r = self.new_rect(self.sprite_width, self.sprite_height, screen_width, screen_height);
            while r.has_intersection(no_spawn_rect) {
                r = self.new_rect(self.sprite_width, self.sprite_height, screen_width, screen_height);
            }
            self.black_holes.push(BlackHole::new(&r));
        }
    }

    pub fn update(&mut self, game: &Game) {
        if self.black_holes.is_empty() {
            self.set_black_holes(
                game.player.w as u32,
                game.player.h as u32,
                game.screen_width, game.screen_height,
                game.black_hole_count
            );
        }

        for black_hole in self.black_holes.iter_mut() {
            if black_hole.last_update.elapsed().unwrap() > ROTATION_UPDATE_MILLIS {
                black_hole.sprite.degrees = (black_hole.sprite.degrees + 10.0).rem_euclid(360.0);
                black_hole.last_update = SystemTime::now();
            }
        }
    }

    pub fn reset(&mut self) {
        self.black_holes.clear();
    }
}
