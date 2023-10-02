use crate::controller::Controller;
use graphics::Context;
use opengl_graphics::{GlGraphics, Texture};
use piston_window::{
    TextureSettings
};
use sprite::Sprite;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use uuid::Uuid;

struct Laser {
    id: Uuid,
    degrees: f64,
    x: f64,
    y: f64,
    m: f64,
    b: f64,
    x_increment: f64,
    y_increment: f64
}

impl Laser {
    pub fn new(degrees: f64, x1: f64, y1: f64, x2: f64, y2: f64, window_width: f64, window_height: f64) -> Self {
        let m = (y2 - y1) / (x2 - x1);
        let mut x_increment = -1.0 * (x1 / 50.0);
        if x1 < x2 {
            x_increment = (window_width - x1) / 50.0;
        }

        let mut y_increment = -1.0 * (y1 / 50.0);
        if y1 < y2 {
            y_increment = (window_height - y1) / 50.0;
        }

        Self {
            id: Uuid::new_v4(),
            degrees: degrees + 90.0,
            x: x1,
            y: y1,
            m,
            b: y1 - (m * x1),
            x_increment,
            y_increment
        }
    }

    pub fn get_id(&self) -> Uuid {
        return self.id;
    }

    pub fn update(&mut self) {
        let prev_x = self.x;
        let prev_y = self.y;
        let x_inc_x = self.x + self.x_increment;
        let x_inc_y = (self.m * (self.x + self.x_increment)) + self.b;
        let y_inc_y = self.y + self.y_increment;
        let y_inc_x = (self.y - self.b) / self.m;

        if (x_inc_x - prev_x).abs() > (y_inc_y - prev_y).abs() {
            self.x = x_inc_x;
            self.y = x_inc_y;
        } else {
            self.x = y_inc_x;
            self.y = y_inc_y;
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
    lasers: HashMap<Uuid, Laser>,
    window_width: f64,
    window_height: f64,
}

impl Lasers {
    pub fn new(window_width: f64, window_height: f64, sprite_path: &str) -> Self {
        let texture = Texture::from_path(Path::new(sprite_path), &TextureSettings::new()).unwrap();
        Self {
            sprite: Sprite::from_texture(Rc::new(texture)),
            lasers: HashMap::new(),
            window_width,
            window_height
        }
    }

    pub fn update(&mut self, controller: Controller, player_x: &f64, player_y: &f64) {
        let right_stick_pos = controller.get_right_stick();
        if right_stick_pos.get_x() != 0.0 || right_stick_pos.get_y() != 0.0 {
            if self.lasers.is_empty() {
                let laser = Laser::new(
                    right_stick_pos.get_degrees(), *player_x, *player_y,
                    right_stick_pos.get_screen_x(), right_stick_pos.get_screen_y(),
                    self.window_width, self.window_height
                );
                self.lasers.insert(laser.id, laser);
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
    }

    pub fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        for laser in self.lasers.values() {
            self.sprite.set_position(laser.x, laser.y);
            self.sprite.set_rotation(laser.degrees);
            self.sprite.draw(ctx.transform, gl);
        }
    }
}
