use graphics::Context;
use opengl_graphics::{
    GlGraphics,
    ImageSize,
    Texture
};
use piston::{
    Size,
};
use piston_window::{
    TextureSettings
};
use sprite::Sprite;
use std::rc::Rc;
use rust_embed::EmbeddedFile;
use crate::game::Game;
use crate::game_sprite::GameSprite;

const PLAYER_DIE_INCREMENT: f64 = 1.0;

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerState {
    NotTowing,
    Towing,
    Dying,
    Dead
}

pub struct Player {
    sprite: GameSprite,
    sprite_texture: Sprite<Texture>,
    start_x: f64,
    start_y: f64,
    start_height: f64,
    start_width: f64,
    state: PlayerState,
    window_width: f64,
    window_height: f64,
}

impl Player {
    pub fn new(window_width: f64, window_height: f64, sprite_file: &EmbeddedFile) -> Self {
        let image = image::load_from_memory(sprite_file.data.as_ref()).unwrap();
        let texture = Texture::from_image(image.as_rgba8().unwrap(), &TextureSettings::new());
        let size = texture.get_size();
        let half_size = Size::from([size.0 / 2, size.1 / 2]);
        let start_x = (window_width / 2.0) - half_size.width;
        let start_y = (window_height / 2.0) - half_size.height;
        Self {
            sprite: GameSprite::new((window_width / 2.0) - half_size.width, (window_height / 2.0) - half_size.height, size.0 as f64, size.1 as f64),
            sprite_texture: Sprite::from_texture(Rc::new(texture)),
            start_x,
            start_y,
            start_height: size.1 as f64,
            start_width: size.0 as f64,
            state: PlayerState::NotTowing,
            window_width,
            window_height
        }
    }

    pub fn get_state(&mut self) -> PlayerState {
        self.state
    }

    pub fn get_sprite(&self) -> GameSprite {
        self.sprite
    }

    pub fn not_towing(&mut self) {
        self.state = PlayerState::NotTowing;
    }

    pub fn towing(&mut self) {
        self.state = PlayerState::Towing;
    }

    pub fn dying(&mut self) {
        self.state = PlayerState::Dying;
        self.sprite.shatter_start();
    }

    pub fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        match self.state {
            PlayerState::Dying => {
                self.sprite.shatter(&mut self.sprite_texture, self.start_width as u32, self.start_height as u32, ctx, gl);
            }
            PlayerState::Dead => {}
            _ => {
                self.sprite_texture.set_src_rect([0.0, 0.0, self.sprite.width, self.sprite.height]);
                self.sprite.draw(&mut self.sprite_texture, ctx, gl);
            }
        }
    }

    pub fn update(&mut self, game: &Game) {
        match self.state {
            PlayerState::Dying => {
                if self.sprite.shatter_update(PLAYER_DIE_INCREMENT, self.window_width, self.window_height) {
                    self.state = PlayerState::Dead;
                }
            }
            PlayerState::Dead => {}
            _ => {
                let left_stick_pos = game.controller.get_left_stick();
                self.sprite.degrees = left_stick_pos.get_degrees() + 90.0;
                self.sprite.set_position(
                    (self.sprite.x + (left_stick_pos.get_x() * 1.0)).min(self.window_width).max(0.0),
                    (self.sprite.y + (left_stick_pos.get_y() * 1.0)).min(self.window_height).max(0.0)
                );
            }
        }
    }

    pub fn reset(&mut self) {
        self.sprite.set_position(self.start_x, self.start_y);
        self.sprite.height = self.start_height;
        self.sprite.width = self.start_width;
        self.state = PlayerState::NotTowing;
    }
}
