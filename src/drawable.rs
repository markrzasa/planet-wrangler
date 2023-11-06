use graphics::{Context};
use opengl_graphics::{GlGraphics};

pub trait Drawable {
    fn draw(&mut self, ctx: Context, gl: &mut GlGraphics);
}
