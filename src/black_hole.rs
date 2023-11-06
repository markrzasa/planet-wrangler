use graphics::Context;
use opengl_graphics::{GlGraphics, Texture};
use piston_window::TextureSettings;
use rust_embed::EmbeddedFile;
use sprite::Sprite;
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use sdl2::rect::Rect;
use crate::drawable::Drawable;
use crate::game_context::GameContext;
use crate::updateable::Updateable;

const ROTATION_UPDATE_MILLIS: Duration = Duration::from_millis(250);

#[derive(Clone, Copy, PartialEq)]
pub enum BlackHoleState {
    Covered,
    Open
}

pub struct BlackHole {
    degrees: f64,
    height: u32,
    last_update: SystemTime,
    state: BlackHoleState,
    width: u32,
    x: f64,
    y: f64
}

impl BlackHole {
    pub fn new(r: &Rect) -> Self {
        Self {
            degrees: 0.0,
            height: r.height(),
            last_update: SystemTime::now(),
            state: BlackHoleState::Open,
            width: r.width(),
            x: r.x() as f64,
            y: r.y() as f64
        }
    }

    pub fn covered(&mut self) {
        self.state = BlackHoleState::Covered;
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, self.width, self.height)
    }

    pub fn get_state(&self) -> BlackHoleState {
        self.state
    }
}

pub struct BlackHoles {
    sprite: Sprite<Texture>,
    black_holes: Vec<BlackHole>
}

impl BlackHoles {
    pub fn new(sprite_file: &EmbeddedFile) -> Self {
        let image = image::load_from_memory(sprite_file.data.as_ref()).unwrap();
        let texture = Texture::from_image(image.as_rgba8().unwrap(), &TextureSettings::new());
        Self {
            sprite: Sprite::from_texture(Rc::new(texture)),
            black_holes: Vec::new()
        }
    }

    pub fn get_black_holes(&mut self) -> &mut Vec<BlackHole> {
        &mut self.black_holes
    }

    pub fn set_black_holes(&mut self, black_holes: &Vec<Rect>) {
        for rect in black_holes.iter() {
            self.black_holes.push(BlackHole::new(rect))
        }
    }
}

impl Drawable for BlackHoles {
    fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        for black_hole in self.black_holes.iter() {
            self.sprite.set_position(black_hole.x, black_hole.y);
            self.sprite.set_rotation(black_hole.degrees);
            self.sprite.draw(ctx.transform, gl);
        }
    }
}

impl Updateable for BlackHoles {
    fn update<'a>(&'a mut self, context: &'a GameContext) -> &GameContext {
        for black_hole in self.black_holes.iter_mut() {
            if black_hole.last_update.elapsed().unwrap() > ROTATION_UPDATE_MILLIS {
                black_hole.degrees = (black_hole.degrees + 10.0).rem_euclid(360.0);
                black_hole.last_update = SystemTime::now();
            }
        }

        context
    }

    fn reset(&mut self) {
        self.black_holes.clear();
    }
}
