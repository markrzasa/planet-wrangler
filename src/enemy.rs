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
use crate::drawable::Drawable;
use crate::game_context::GameContext;
use crate::updateable::Updateable;

const ENEMY_MOVE_INCREMENT: f64 = 0.25;
const ENEMY_DIE_INCREMENT: u32 = 1;
const FRAME_DURATION_MILLIS: Duration = Duration::from_millis(100);
const MAX_ENEMIES: usize = 75;
const WAIT_TO_SPAWN_DURATION: Duration = Duration::from_millis(2000);

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyState {
    Alive,
    Dying,
    Dead
}

pub struct Enemy {
    id: Uuid,
    x: f64,
    y: f64,
    height: u32,
    width: u32,
    sprite_index: u32,
    state: EnemyState,
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
            state: EnemyState::Alive,
            frames,
            last_frame_change: SystemTime::now()
        }
    }

    pub fn dying(&mut self) {
        self.state = EnemyState::Dying;
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

    pub fn get_state(&self) -> EnemyState {
        self.state
    }

    fn update(&mut self, player: Rect, screen_height: f64, screen_width: f64) {
        match self.state {
            EnemyState::Alive => {
                if self.last_frame_change.elapsed().unwrap() >= FRAME_DURATION_MILLIS {
                    self.sprite_index = (self.sprite_index + 1).rem_euclid(self.frames);
                    self.last_frame_change = SystemTime::now();
                }
                if (player.x as f64) < self.x {
                    self.x = self.x - ENEMY_MOVE_INCREMENT;
                } else {
                    self.x = self.x + ENEMY_MOVE_INCREMENT;
                }

                if (player.y as f64) < self.y {
                    self.y = self.y - ENEMY_MOVE_INCREMENT;
                } else {
                    self.y = self.y + ENEMY_MOVE_INCREMENT;
                }
            }
            EnemyState::Dying => {
                self.x = self.x - ENEMY_DIE_INCREMENT as f64;
                self.y = self.y - ENEMY_DIE_INCREMENT as f64;
                self.width = self.width + (ENEMY_DIE_INCREMENT * 2);
                self.height = self.height + (ENEMY_DIE_INCREMENT * 2);
                if (self.x <= 0.0) && (self.y <= 0.0) && ((self.x + self.width as f64) >= screen_width) && ((self.y + self.height as f64) >= screen_height) {
                    self.state = EnemyState::Dead;
                }
            }
            EnemyState::Dead => {}
        }
    }
}

enum EnemiesState {
    Running,
    WaitingToSpawn,
    WaitingForSpawnPoints
}

pub struct Enemies {
    enemies: HashMap<Uuid, Enemy>,
    last_enemy: SystemTime,
    sprite: Sprite<Texture>,
    sprite_height: u32,
    sprite_width: u32,
    sprite_frames: u32,
    state: EnemiesState,
    wait_start: SystemTime
}

impl Enemies {
    pub fn new(sprite_file: &EmbeddedFile, sprite_frames: u32) -> Self {
        let image = image::load_from_memory(sprite_file.data.as_ref()).unwrap();
        let texture = Texture::from_image(image.as_rgba8().unwrap(), &TextureSettings::new());
        let sprite_height = texture.get_height();
        let sprite_width = texture.get_width() / sprite_frames;
        Self {
            enemies: HashMap::new(),
            last_enemy: SystemTime::now(),
            sprite: Sprite::from_texture(Rc::new(texture)),
            sprite_frames,
            sprite_height,
            sprite_width,
            state: EnemiesState::WaitingForSpawnPoints,
            wait_start: SystemTime::now()
        }
    }

    pub fn get_enemies(&mut self) -> &mut HashMap<Uuid, Enemy> {
        &mut self.enemies
    }

    pub fn remove(&mut self, id: &Uuid) {
        self.enemies.remove(id);
    }
}

impl Drawable for Enemies {
    fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        for enemy in self.enemies.values() {
            match enemy.get_state() {
                EnemyState::Alive => {
                    self.sprite.set_src_rect([
                        self.sprite_width as f64 * enemy.get_sprite_index() as f64,
                        0.0,
                        self.sprite_width as f64,
                        self.sprite_height as f64
                    ]);
                    self.sprite.set_position(enemy.x, enemy.y);
                    self.sprite.draw(ctx.transform, gl);
                }
                EnemyState::Dying => {
                    let width = self.sprite_width as f64 / 2.0;
                    let height = self.sprite_height as f64 / 2.0;
                    self.sprite.set_src_rect([0.0, 0.0, width, height]);
                    self.sprite.set_position(enemy.x, enemy.y);
                    self.sprite.draw(ctx.transform, gl);

                    self.sprite.set_src_rect([width, 0.0, width, height]);
                    self.sprite.set_position(enemy.x + enemy.width as f64 - width, enemy.y);
                    self.sprite.draw(ctx.transform, gl);

                    self.sprite.set_src_rect([0.0, height, width, height]);
                    self.sprite.set_position(enemy.x + enemy.width as f64 - width, enemy.y + enemy.height as f64 - height);
                    self.sprite.draw(ctx.transform, gl);

                    self.sprite.set_src_rect([width, height, width, height]);
                    self.sprite.set_position(enemy.x, enemy.y + enemy.height as f64 - height);
                    self.sprite.draw(ctx.transform, gl);
                }
                _ => {}
            }
        }
    }
}

impl Updateable for Enemies {
    fn update<'s>(&mut self, context: &'s GameContext) -> &'s GameContext {
        match self.state {
            EnemiesState::Running => {
                for (_, e) in self.enemies.iter_mut() {
                    e.update(context.get_player(), context.get_screen_height(), context.get_screen_width());
                }

                if self.last_enemy.elapsed().unwrap().as_millis() > 500 && self.enemies.len() < MAX_ENEMIES {
                    let spawn_points = context.get_spawn_points();
                    let i = rand::thread_rng().gen_range(0, spawn_points.len());
                    let p = spawn_points.get(i).unwrap();
                    let enemy = Enemy::new(
                        p.x as f64, p.y as f64,
                        self.sprite_width, self.sprite_height, self.sprite_frames
                    );
                    self.enemies.insert(enemy.get_id(), enemy);
                    self.last_enemy = SystemTime::now();
                }
            }
            EnemiesState::WaitingToSpawn => {
                if self.wait_start.elapsed().unwrap() > WAIT_TO_SPAWN_DURATION {
                    self.state = EnemiesState::Running;
                }
            }
            EnemiesState::WaitingForSpawnPoints => {
                if !context.get_spawn_points().is_empty() {
                    self.state = EnemiesState::WaitingToSpawn;
                    self.wait_start = SystemTime::now();
                }
            }
        }

        context
    }

    fn reset(&mut self) {
        self.enemies.clear();
        self.state = EnemiesState::WaitingForSpawnPoints;
    }
}
