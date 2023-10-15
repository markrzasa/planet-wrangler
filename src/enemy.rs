use std::collections::HashMap;
use std::rc::Rc;
use std::time::SystemTime;
use graphics::{Context};
use opengl_graphics::{GlGraphics, Texture};
use piston_window::TextureSettings;
use rand::Rng;
use rust_embed::EmbeddedFile;
use sprite::Sprite;
use uuid::Uuid;

struct Enemy {
    id: Uuid,
    x: f64,
    y: f64,
}

impl Enemy {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            x,
            y
        }
    }

    pub fn get_id(&self) -> Uuid {
        return self.id;
    }
}

pub struct Enemies {
    sprite: Sprite<Texture>,
    enemies: HashMap<Uuid, Enemy>,
    last_enemy: SystemTime,
    window_width: f64,
    window_height: f64,
}

impl Enemies {
    pub fn new(window_width: f64, window_height: f64, sprite_file: &EmbeddedFile) -> Self {
        let image = image::load_from_memory(sprite_file.data.as_ref()).unwrap();
        let texture = Texture::from_image(image.as_rgba8().unwrap(), &TextureSettings::new());
        Self {
            sprite: Sprite::from_texture(Rc::new(texture)),
            enemies: HashMap::new(),
            last_enemy: SystemTime::now(),
            window_width,
            window_height
        }
    }

    pub fn spawn_enemy(&mut self) {
        if self.last_enemy.elapsed().unwrap().as_millis() > 500 && self.enemies.len() < 50 {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0, self.window_width as i32);
            let y = rng.gen_range(0, self.window_height as i32);
            let enemy = Enemy::new(x as f64, y as f64);
            self.enemies.insert(enemy.get_id(), enemy);
            self.last_enemy = SystemTime::now();
        }
    }

    pub fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        for enemy in self.enemies.values() {
            self.sprite.set_position(enemy.x, enemy.y);
            self.sprite.draw(ctx.transform, gl);
        }
    }
}
