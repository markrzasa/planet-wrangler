mod controller;
mod game;
mod laser;
mod player;
mod enemy;

extern crate graphics;
extern crate image;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;
extern crate sprite;
extern crate rust_embed;

use graphics::{clear};
use opengl_graphics::{GlGraphics, OpenGL, };
use piston::{Events, EventSettings, RenderEvent};
use piston::window::{
    WindowSettings,
};
use piston_window::{color, Window};
use rust_embed::RustEmbed;
use sdl2_window::Sdl2Window;
use crate::game::Game;

const WINDOW_HEIGHT: f64 = 1000.0;
const WINDOW_WIDTH: f64 = 1000.0;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;


fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window = WindowSettings::new("Planet Wrangler", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .fullscreen(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let mut font = Assets::get("PressStart2PRegular.ttf").unwrap();
    let mut enemy_png = Assets::get("enemy.png").unwrap();
    let mut hero_png = Assets::get("hero.png").unwrap();
    let mut laser_png = Assets::get("laser.png").unwrap();

    let mut game = Game::new(
        window.size().width, window.size().height,
        &mut font, &mut hero_png, &mut laser_png, &mut enemy_png
    );

    let mut events = Events::new(EventSettings::new());
    while let Some(mut event) = events.next(&mut window) {
        game.update(&mut event);

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |ctx, gl| {
                clear(color::GRAY, gl);
                game.draw(ctx, gl);
            });
        }
    }
}
