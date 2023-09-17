mod player;
mod controller;

extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;
extern crate sprite;

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
use sdl2_window::Sdl2Window;
use crate::controller::Controller;
use crate::player::Player;

const WINDOW_HEIGHT: f64 = 1000.0;
const WINDOW_WIDTH: f64 = 1000.0;

enum GameState {
    Starting,
    Running,
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window = WindowSettings::new("Planet Wrangler", [WINDOW_WIDTH, WINDOW_WIDTH])
        .exit_on_esc(true).graphics_api(opengl).build().unwrap();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let mut glyphs = GlyphCache::new("./assets/PressStart2PRegular.ttf", (), TextureSettings::new()).unwrap();

    let mut controller = Controller::new();
    let mut player = Player::new(WINDOW_WIDTH, WINDOW_HEIGHT, "./assets/hero.png");

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

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |ctx, gl| {
                clear(color::GRAY, gl);

                let transform = ctx.transform.trans(10.0, 20.0);
                text::Text::new_color(color::YELLOW, 10).draw(
                    &format!("{:.3}, {:.3}", controller.get_left_stick().get_x(), controller.get_left_stick().get_y()),
                    &mut glyphs, &ctx.draw_state, transform, gl
                ).unwrap();

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
                    }
                }
            });
        }
    }
}