use graphics::{Context, text, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::{Button, ControllerAxisEvent, Event, ReleaseEvent};
use piston_window::{color, TextureSettings};
use rust_embed::{EmbeddedFile};

use crate::controller::Controller;
use crate::enemy::Enemies;
use crate::laser::Lasers;
use crate::player::Player;

#[derive(PartialEq)]
enum GameState {
    Starting,
    Running,
}

pub struct GameContext {
    controller: Controller,
    player_x: f64,
    player_y: f64
}

impl GameContext {
    fn new(controller: &mut Controller, player_x: f64, player_y: f64) -> Self {
        Self {
            controller: *controller,
            player_x,
            player_y
        }
    }

    pub fn get_controller(&self) -> Controller {
        self.controller
    }

    pub fn get_player_x(&self) -> f64 {
        self.player_x
    }

    pub fn get_player_y(&self) -> f64 {
        self.player_y
    }
}

pub struct Game<'a> {
    controller: Controller,
    glyphs: GlyphCache<'a>,
    lasers: Lasers,
    player: Player,
    enemies: Enemies,
    state: GameState,
    window_width: f64,
    window_height: f64,
}

impl<'a> Game<'a> {
    pub fn new(
        window_width: f64, window_height: f64, font: &'a mut EmbeddedFile, hero_png: &mut EmbeddedFile, laser_png: &mut EmbeddedFile, enemy_png: &mut EmbeddedFile
    ) -> Self {
        Self {
            controller: Controller::new(window_width, window_height),
            glyphs: GlyphCache::from_bytes(font.data.as_ref(), (), TextureSettings::new()).unwrap(),
            lasers: Lasers::new(window_width, window_height, laser_png),
            player: Player::new(window_width, window_height, hero_png),
            enemies: Enemies::new(window_width, window_height, enemy_png),
            state: GameState::Starting,
            window_width,
            window_height
        }
    }

    pub fn update(&mut self, event: &mut Event) {
        if let Some(args) = event.controller_axis_args() {
            self.controller.update(args);
        }
        if let Some(button) = event.release_args() {
            match button {
                Button::Controller(_) => self.state = GameState::Running,
                _ => {},
            }
        }

        if self.state == GameState::Running {
            self.enemies.spawn_enemy();

            let mut game_context = &GameContext::new(&mut self.controller, self.player.get_x(), self.player.get_y());
            game_context = self.player.update(game_context);
            self.lasers.update(game_context);
        }
    }

    pub fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        match self.state {
            GameState::Starting => {
                let transform = ctx.transform.trans((self.window_width / 2.0) - 250.0, (self.window_height / 2.0) - 14.0);
                text::Text::new_color(color::BLUE, 14).draw(
                    "Press a button to start",
                    &mut self.glyphs, &ctx.draw_state, transform, gl
                ).unwrap();
            }
            _ => {
                self.player.draw(ctx, gl);
                self.lasers.draw(ctx, gl);
                self.enemies.draw(ctx, gl);
            }
        }
    }
}
