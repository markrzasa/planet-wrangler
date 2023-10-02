use crate::controller::Controller;
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
use std::path::Path;
use std::rc::Rc;

pub struct Player {
    degrees: f64,
    pub sprite: Sprite<Texture>,
    x: f64,
    y: f64,
    window_width: f64,
    window_height: f64,
}

impl Player {
    pub fn new(window_width: f64, window_height: f64, sprite_path: &str) -> Self {
        let texture = Texture::from_path(Path::new(sprite_path), &TextureSettings::new()).unwrap();
        let size = texture.get_size();
        let half_size = Size::from([size.0 / 2, size.1 / 2]);
        Self {
            degrees: 0.0,
            sprite: Sprite::from_texture(Rc::new(texture)),
            x: (window_width / 2.0) - half_size.width,
            y: (window_height / 2.0) - half_size.height,
            window_width,
            window_height
        }
    }

    pub fn get_x(&mut self) -> f64 {
        self.x
    }

    pub fn get_y(&mut self) -> f64 {
        self.y
    }

    pub fn update(&mut self, controller: Controller) {
        let left_stick_pos = controller.get_left_stick();
        self.degrees = left_stick_pos.get_degrees() + 90.0;
        let prev_x = self.x;
        let prev_y = self.y;
        self.x = (prev_x + (left_stick_pos.get_x() * 1.0)).min(self.window_width).max(0.0);
        self.y = (prev_y + (left_stick_pos.get_y() * 1.0)).min(self.window_height).max(0.0);
    }

    pub fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        self.sprite.set_position(self.x, self.y);
        self.sprite.set_rotation(self.degrees);
        self.sprite.draw(ctx.transform, gl);
    }
}
