use graphics::Context;
use opengl_graphics::{GlGraphics, Texture};
use sdl2::rect::Rect;
use sprite::Sprite;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq)]
pub struct GameSprite {
    id: Uuid,
    shatter_x: f64,
    shatter_y: f64,
    shatter_width: f64,
    shatter_height: f64,
    pub degrees: f64,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64
}

impl GameSprite {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            shatter_x: 0.0,
            shatter_y: 0.0,
            shatter_width: 0.0,
            shatter_height: 0.0,
            degrees: 0.0,
            x, y, width, height
        }
    }

    pub fn from_rect(r: &Rect) -> Self {
        Self {
            id: Uuid::new_v4(),
            shatter_x: 0.0,
            shatter_y: 0.0,
            shatter_width: 0.0,
            shatter_height: 0.0,
            degrees: 0.0,
            x: r.x as f64,
            y: r.y as f64,
            width: r.w as f64,
            height: r.h as f64
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_position(&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, self.width as u32, self.height as u32)
    }

    pub fn set_position(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn draw(&mut self, sprite: &mut Sprite<Texture>, ctx: Context, gl: &mut GlGraphics) {
        sprite.set_position(self.x, self.y);
        sprite.set_rotation(self.degrees);
        sprite.draw(ctx.transform, gl);
    }

    pub fn shatter(&mut self, sprite: &mut Sprite<Texture>, sprite_width: u32, sprite_height: u32, ctx: Context, gl: &mut GlGraphics) {
        let width = sprite_width as f64 / 2.0;
        let height = sprite_height as f64 / 2.0;
        sprite.set_src_rect([0.0, 0.0, width, height]);
        sprite.set_position(self.shatter_x, self.shatter_y);
        sprite.draw(ctx.transform, gl);

        sprite.set_src_rect([width, 0.0, width, height]);
        sprite.set_position(self.shatter_x + self.shatter_width - width, self.shatter_y);
        sprite.draw(ctx.transform, gl);

        sprite.set_src_rect([0.0, height, width, height]);
        sprite.set_position(self.shatter_x + self.shatter_width - width, self.shatter_y + self.shatter_height - height);
        sprite.draw(ctx.transform, gl);

        sprite.set_src_rect([width, height, width, height]);
        sprite.set_position(self.shatter_x, self.shatter_y + self.shatter_height - height);
        sprite.draw(ctx.transform, gl);
    }

    pub fn shatter_start(&mut self) {
        self.shatter_x = self.x;
        self.shatter_y = self.y;
        self.shatter_width = self.width;
        self.shatter_height = self.height;
    }

    pub fn shatter_update(&mut self, increment: f64, screen_width: f64, screen_height: f64) -> bool {
        self.shatter_x -= increment;
        self.shatter_y -= increment;
        self.shatter_width += increment * 2.0;
        self.shatter_height += increment * 2.0;

        (self.shatter_x <= 0.0) && (self.shatter_y <= 0.0) && ((self.shatter_x + self.shatter_width) >= screen_width) && ((self.shatter_y + self.shatter_height) >= screen_height)
    }
}
