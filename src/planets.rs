use graphics::{Context, ImageSize};
use opengl_graphics::{GlGraphics, Texture};
use piston_window::TextureSettings;
use rust_embed::EmbeddedFile;
use sprite::Sprite;
use std::collections::HashMap;
use std::rc::Rc;
use rand::Rng;
use sdl2::rect::Rect;
use uuid::Uuid;
use crate::game_context::GameContext;
use crate::game_sprite::GameSprite;

#[derive(Clone, Copy, PartialEq)]
pub enum PlanetState {
    InPlace,
    NotTowed,
    Towed
}

pub struct Planet {
    sprite: GameSprite,
    sprite_index: u32,
    state: PlanetState,
}

impl Planet {
    pub fn new(x: f64, y: f64, sprite_index: u32, width: u32, height: u32) -> Self {
        Self {
            sprite_index,
            sprite: GameSprite::new(x, y, width as f64, height as f64),
            state: PlanetState::NotTowed
        }
    }

    pub fn get_sprite(&self) -> GameSprite {
        self.sprite
    }

    pub fn get_state(&self) -> PlanetState {
        self.state
    }

    pub fn in_place(&mut self, rect: Rect)
    {
        self.sprite.x = rect.x() as f64;
        self.sprite.y = rect.y() as f64;
        self.state = PlanetState::InPlace;
    }

    pub fn not_towed(&mut self) {
        self.state = PlanetState::NotTowed;
    }

    pub fn towed(&mut self) {
        self.state = PlanetState::Towed;
    }

    fn update(&mut self, player: Rect) {
        if self.state == PlanetState::Towed {
            self.sprite.x = player.x as f64;
            self.sprite.y = player.y as f64;
        }
    }
}

pub struct Planets {
    done_sprite: Sprite<Texture>,
    frames: u32,
    planet_sprite: Sprite<Texture>,
    planets: HashMap<Uuid, Planet>,
    sprite_height: u32,
    sprite_width: u32,
    window_height: f64,
    window_width: f64
}

impl Planets {
    pub fn new(
        done_file: &EmbeddedFile,
        planet_file: &EmbeddedFile,
        frames: u32, window_width: f64, window_height: f64
    ) -> Self {
        let done_image = image::load_from_memory(done_file.data.as_ref()).unwrap();
        let done_texture = Texture::from_image(done_image.as_rgba8().unwrap(), &TextureSettings::new());
        let planet_image = image::load_from_memory(planet_file.data.as_ref()).unwrap();
        let planet_texture = Texture::from_image(planet_image.as_rgba8().unwrap(), &TextureSettings::new());
        let sprite_width = planet_texture.get_width() / frames;
        let sprite_height = planet_texture.get_height();
        Self {
            done_sprite: Sprite::from_texture(Rc::new(done_texture)),
            frames,
            planet_sprite: Sprite::from_texture(Rc::new(planet_texture)),
            planets: HashMap::new(),
            sprite_height,
            sprite_width,
            window_height,
            window_width
        }
    }

    pub fn get_planets(&mut self) -> &mut HashMap<Uuid, Planet> {
        &mut self.planets
    }

    fn new_rect(&mut self) -> Rect {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, self.window_width as u32);
        let y = rng.gen_range(0, self.window_height as u32);

        Rect::new(x as i32, y as i32, x + self.sprite_width, y + self.sprite_height)
    }

    pub fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        for planet in self.planets.values_mut() {
            self.planet_sprite.set_src_rect([
                self.sprite_width as f64 * planet.sprite_index as f64,
                0.0,
                self.sprite_width as f64,
                self.sprite_height as f64
            ]);
            planet.sprite.draw(&mut self.planet_sprite, ctx, gl);
            if planet.get_state() == PlanetState::InPlace {
                self.done_sprite.set_position(planet.sprite.x, planet.sprite.y);
                self.done_sprite.draw(ctx.transform, gl);
            }
        }
    }

    pub fn update<'a>(&'a mut self, context: &'a GameContext) -> &GameContext {
        if self.planets.is_empty() {
            let num_planets = context.get_spawn_points().len();
            for i in 0..num_planets {
                let mut got_rect = false;
                let mut r = self.new_rect();
                while !got_rect {
                    got_rect = true;
                    for p in context.get_spawn_points().iter() {
                        if r.has_intersection(*p) {
                            got_rect = false;
                            r = self.new_rect();
                            break;
                        }
                    }
                }
                let p = Planet::new(
                    r.x as f64, r.y as f64, i.rem_euclid(self.frames as usize) as u32,
                    self.sprite_width, self.sprite_height
                );
                self.planets.insert(p.sprite.get_id(), p);
            }
        } else {
            for (_, planet) in self.planets.iter_mut() {
                planet.update(context.get_player());
            }
        }

        context
    }

    pub fn reset(&mut self) {
        self.planets.clear();
    }
}
