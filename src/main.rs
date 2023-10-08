mod player;
mod controller;
mod laser;

extern crate graphics;
extern crate image;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;
extern crate sprite;
extern crate rust_embed;

use crate::controller::Controller;
use crate::laser::Lasers;
use crate::player::Player;
use graphics::{clear, text, Transformed};
use opengl_graphics::{
    GlGraphics,
    GlyphCache,
    OpenGL,
};
use piston::{Button, ControllerAxisEvent, Events, EventSettings, ReleaseEvent, RenderEvent};
use piston::window::{
    WindowSettings,
};
use piston_window::{
    color,
    TextureSettings
};
use rust_embed::RustEmbed;
use sdl2_window::Sdl2Window;

const WINDOW_HEIGHT: f64 = 1000.0;
const WINDOW_WIDTH: f64 = 1000.0;

enum GameState {
    Starting,
    Running,
}

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window = WindowSettings::new("Planet Wrangler", [WINDOW_WIDTH, WINDOW_WIDTH])
        .exit_on_esc(true).graphics_api(opengl).build().unwrap();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let mut controller = Controller::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let font = Assets::get("PressStart2PRegular.ttf").unwrap();
    let mut glyphs = GlyphCache::from_bytes(font.data.as_ref(), (), TextureSettings::new()).unwrap();

    let hero_png = Assets::get("hero.png").unwrap();
    let mut player = Player::new(WINDOW_WIDTH, WINDOW_HEIGHT, &hero_png);
    let laser_png = Assets::get("laser.png").unwrap();
    let mut lasers = Lasers::new(WINDOW_WIDTH, WINDOW_HEIGHT, &laser_png);
    let mut game_state = GameState::Starting;

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.controller_axis_args() {
            controller.update(args);
        }
        if let Some(button) = event.release_args() {
            match button {
                Button::Controller(_) => game_state = GameState::Running,
                _ => {},
            }
        }

        player.update(controller);
        lasers.update(controller, &player.get_x(), &player.get_y());

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |ctx, gl| {
                clear(color::GRAY, gl);

                match game_state {
                    GameState::Starting => {
                        let transform = ctx.transform.trans((WINDOW_WIDTH / 2.0) - 250.0, (WINDOW_HEIGHT / 2.0) - 14.0);
                        text::Text::new_color(color::BLUE, 14).draw(
                            "Press a button to start",
                            &mut glyphs, &ctx.draw_state, transform, gl
                        ).unwrap();
                    }
                    _ => {
                        player.draw(ctx, gl);
                        lasers.draw(ctx, gl);
                    }
                }
            });
        }
    }
}
