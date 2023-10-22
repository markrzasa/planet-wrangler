use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use graphics::{Context, ImageSize};
use opengl_graphics::{GlGraphics, Texture};
use piston_window::TextureSettings;
use rand::Rng;
use rust_embed::EmbeddedFile;
use sdl2::rect::Rect;
use sprite::Sprite;
use uuid::Uuid;
use crate::game::GameContext;

const ENEMY_MOVE_INCREMENT: f64 = 0.25;
const FRAME_DURATION_MILLIS: Duration = Duration::from_millis(100);
const MAX_ENEMIES: usize = 75;

pub struct Enemy {
    id: Uuid,
    x: f64,
    y: f64,
    height: u32,
    width: u32,
    sprite_index: u32,
    frames: u32,
    last_frame_change: SystemTime
}

impl Enemy {
    pub fn new(x: f64, y: f64, width: u32, height: u32, frames: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            x,
            y,
            height,
            width,
            sprite_index: 0,
            frames,
            last_frame_change: SystemTime::now()
        }
    }

    pub fn get_id(&self) -> Uuid {
        return self.id;
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, self.width, self.height)
    }

    pub fn get_sprite_index(&self) -> u32 {
        self.sprite_index
    }

    fn update(&mut self, player_x: f64, player_y: f64) {
        if self.last_frame_change.elapsed().unwrap() >= FRAME_DURATION_MILLIS {
            self.sprite_index = (self.sprite_index + 1).rem_euclid(self.frames);
        }
        if player_x < self.x {
            self.x = self.x - ENEMY_MOVE_INCREMENT;
        } else {
            self.x = self.x + ENEMY_MOVE_INCREMENT;
        }

        if player_y < self.y {
            self.y = self.y - ENEMY_MOVE_INCREMENT;
        } else {
            self.y = self.y + ENEMY_MOVE_INCREMENT;
        }
    }
}

pub struct Enemies {
    sprite: Sprite<Texture>,
    enemies: HashMap<Uuid, Enemy>,
    sprite_height: u32,
    sprite_width: u32,
    sprite_frames: u32,
    last_enemy: SystemTime,
    window_width: f64,
    window_height: f64,
}

impl Enemies {
    pub fn new(window_width: f64, window_height: f64, sprite_file: &EmbeddedFile, sprite_frames: u32) -> Self {
        let image = image::load_from_memory(sprite_file.data.as_ref()).unwrap();
        let texture = Texture::from_image(image.as_rgba8().unwrap(), &TextureSettings::new());
        let sprite_height = texture.get_height();
        let sprite_width = texture.get_width() / sprite_frames;
        Self {
            sprite: Sprite::from_texture(Rc::new(texture)),
            sprite_height,
            sprite_width,
            sprite_frames,
            enemies: HashMap::new(),
            last_enemy: SystemTime::now(),
            window_width,
            window_height
        }
    }

    pub fn get_enemies(&mut self) -> &HashMap<Uuid, Enemy> {
        &self.enemies
    }

    pub fn remove(&mut self, id: &Uuid) {
        self.enemies.remove(id);
    }

    fn enemy_rect(&self) -> Rect {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, self.window_width as u32);
        let y = rng.gen_range(0, self.window_height as u32);

        Rect::new(x as i32, y as i32, x + self.sprite_width, y + self.sprite_height)
    }

    fn get_spawn_coordinates(&self, no_spawn_rect: &Rect) -> (f64, f64) {
        let mut enemy_rect = self.enemy_rect();
        while enemy_rect.has_intersection(*no_spawn_rect) {
            enemy_rect = self.enemy_rect();
        }

        (enemy_rect.x as f64, enemy_rect.y as f64)
    }

    pub fn update<'s>(&mut self, context: &'s GameContext) -> &'s GameContext {
        for enemy in self.enemies.values_mut() {
            enemy.update(context.get_player_x(), context.get_player_y());
        }

        if self.last_enemy.elapsed().unwrap().as_millis() > 500 && self.enemies.len() < MAX_ENEMIES {
            let x = (context.get_player_x() - context.get_player_width()) as i32;
            let y = (context.get_player_y() - context.get_player_height()) as i32;
            let w = (context.get_player_x() + context.get_player_width()) as u32;
            let h = (context.get_player_y() + context.get_player_height()) as u32;
            let no_spawn_rect = Rect::new(x, y, w, h);
            let spawn_coords = self.get_spawn_coordinates(&no_spawn_rect);
            let enemy = Enemy::new(
                spawn_coords.0, spawn_coords.1, self.sprite_width, self.sprite_height, self.sprite_frames
            );
            self.enemies.insert(enemy.get_id(), enemy);
            self.last_enemy = SystemTime::now();
        }

        context
    }

    pub fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        for enemy in self.enemies.values() {
            self.sprite.set_src_rect([
                self.sprite_width as f64 * enemy.get_sprite_index() as f64,
                0.0,
                self.sprite_width as f64,
                self.sprite_height as f64
            ]);
            self.sprite.set_position(enemy.x, enemy.y);
            self.sprite.draw(ctx.transform, gl);
        }
    }

    pub fn reset(&mut self) {
        self.enemies.clear();
    }
}
