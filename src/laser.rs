use graphics::{Context, ImageSize};
use opengl_graphics::{GlGraphics, Texture};
use piston_window::TextureSettings;
use sprite::Sprite;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::SystemTime;
use rust_embed::EmbeddedFile;
use sdl2::rect::Rect;
use uuid::Uuid;
use crate::drawable::Drawable;
use crate::game_context::GameContext;
use crate::updateable::Updateable;

pub struct Laser {
    id: Uuid,
    degrees: f64,
    x: f64,
    y: f64,
    m: f64,
    b: f64,
    x_increment: f64,
    y_increment: f64,
    vertical: bool,
    height: u32,
    width: u32
}

impl Laser {
    pub fn new(degrees: f64, x1: f64, y1: f64, x2: f64, y2: f64, width: u32, height: u32) -> Self {
        let m = (y2 - y1) / (x2 - x1);
        let mut x_increment = -1.0;
        if x1 < x2 {
            x_increment = 1.0;
        }

        let mut y_increment = -1.0;
        if y1 < y2 {
            y_increment = 1.0;
        }

        Self {
            id: Uuid::new_v4(),
            degrees: degrees + 90.0,
            x: x1,
            y: y1,
            m,
            b: y1 - (m * x1),
            x_increment,
            y_increment,
            vertical: (x2 - x1).abs() < (y2 - y1).abs(),
            height,
            width
        }
    }

    pub fn get_id(&self) -> Uuid {
        return self.id;
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, self.width, self.height)
    }

    pub fn update(&mut self) {
        if self.vertical {
            self.y = self.y + self.y_increment;
            if self.m.abs() != f64::INFINITY {
                self.x = (self.y - self.b) / self.m;
            }
        } else {
            self.x = self.x + self.x_increment;
            self.y = (self.m * self.x) + self.b;
        }
    }

    pub fn is_off_screen(&self, window_width: f64, window_height: f64) -> bool {
        if self.x < 0.0 || self.x > window_width {
            return true;
        }

        if self.y < 0.0 || self.y > window_height {
            return true;
        }

        false
    }
}

pub struct Lasers {
    sprite: Sprite<Texture>,
    sprite_height: u32,
    sprite_width: u32,
    lasers: HashMap<Uuid, Laser>,
    last_laser: SystemTime,
    window_width: f64,
    window_height: f64,
}

impl Lasers {
    pub fn new(window_width: f64, window_height: f64, sprite_file: &EmbeddedFile) -> Self {
        let image = image::load_from_memory(sprite_file.data.as_ref()).unwrap();
        let texture = Texture::from_image(image.as_rgba8().unwrap(), &TextureSettings::new());
        let sprite_height = texture.get_height();
        let sprite_width = texture.get_width();
        Self {
            sprite: Sprite::from_texture(Rc::new(texture)),
            sprite_height,
            sprite_width,
            lasers: HashMap::new(),
            last_laser: SystemTime::now(),
            window_width,
            window_height
        }
    }

    pub fn get_lasers(&mut self) -> &HashMap<Uuid, Laser> {
        &self.lasers
    }

    pub fn remove(&mut self, id: &Uuid) {
        self.lasers.remove(id);
    }
}

impl Drawable for Lasers {
    fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        for laser in self.lasers.values() {
            self.sprite.set_position(laser.x, laser.y);
            self.sprite.set_rotation(laser.degrees);
            self.sprite.draw(ctx.transform, gl);
        }
    }
}

impl Updateable for Lasers {
    fn update<'a>(&'a mut self, context: &'a GameContext) -> &GameContext {
        let right_stick_pos = context.get_controller().get_right_stick();
        if right_stick_pos.get_x() != 0.0 || right_stick_pos.get_y() != 0.0 {
            if self.lasers.len() <= 10 && self.last_laser.elapsed().unwrap().as_millis() > 100 {
                let player_x = context.get_player().x as f64;
                let player_y = context.get_player().y as f64;
                let laser = Laser::new(
                    right_stick_pos.get_degrees(), player_x, player_y,
                    player_x + (right_stick_pos.get_screen_x() - (self.window_width / 2.0)),
                    player_y + (right_stick_pos.get_screen_y() - (self.window_height / 2.0)),
                    self.sprite_width, self.sprite_height
                );
                self.lasers.insert(laser.id, laser);
                self.last_laser = SystemTime::now();
            }
        }

        let mut to_remove = vec!();
        for laser in self.lasers.values_mut() {
            if laser.is_off_screen(self.window_width, self.window_height) {
                to_remove.push(laser.get_id());
            } else {
                laser.update();
            }
        }
        for id in to_remove.iter() {
            self.lasers.remove(id);
        }

        context
    }

    fn reset(&mut self) {
        self.lasers.clear();
    }
}
