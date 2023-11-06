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
use sdl2::rect::Rect;
use crate::drawable::Drawable;
use crate::game_context::GameContext;
use crate::updateable::Updateable;

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerState {
    NotTowing,
    Towing
}

pub struct Player {
    degrees: f64,
    sprite: Sprite<Texture>,
    start_x: f64,
    start_y: f64,
    state: PlayerState,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
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
            degrees: 0.0,
            sprite: Sprite::from_texture(Rc::new(texture)),
            start_x,
            start_y,
            state: PlayerState::NotTowing,
            x: (window_width / 2.0) - half_size.width,
            y: (window_height / 2.0) - half_size.height,
            width: size.0 as f64,
            height: size.1 as f64,
            window_width,
            window_height
        }
    }

    pub fn get_state(&mut self) -> PlayerState {
        self.state
    }

    pub fn get_x(&mut self) -> f64 {
        self.x
    }

    pub fn get_y(&mut self) -> f64 {
        self.y
    }

    pub fn get_width(&mut self) -> f64 {
        self.width
    }

    pub fn get_height(&mut self) -> f64 {
        self.height
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, self.width as u32, self.height as u32)
    }

    pub fn not_towing(&mut self) {
        self.state = PlayerState::NotTowing;
    }

    pub fn towing(&mut self) {
        self.state = PlayerState::Towing;
    }
}

impl Drawable for Player {
    fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        self.sprite.set_position(self.x, self.y);
        self.sprite.set_rotation(self.degrees);
        self.sprite.draw(ctx.transform, gl);
    }
}

impl Updateable for Player {
    fn update<'a>(&'a mut self, context: &'a GameContext) -> &GameContext {
        let left_stick_pos = context.get_controller().get_left_stick();
        self.degrees = left_stick_pos.get_degrees() + 90.0;
        let prev_x = self.x;
        let prev_y = self.y;
        self.x = (prev_x + (left_stick_pos.get_x() * 1.0)).min(self.window_width).max(0.0);
        self.y = (prev_y + (left_stick_pos.get_y() * 1.0)).min(self.window_height).max(0.0);

        context
    }

    fn reset(&mut self) {
        self.x = self.start_x;
        self.y = self.start_y;
        self.state = PlayerState::NotTowing;
    }
}
