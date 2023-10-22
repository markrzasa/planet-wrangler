use graphics::{Context, text, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache, ImageSize, Texture};
use piston::{Button, ControllerAxisEvent, Event, ReleaseEvent};
use piston_window::{color, TextureSettings};
use rust_embed::{EmbeddedFile};
use uuid::Uuid;

use crate::controller::Controller;
use crate::enemy::Enemies;
use crate::laser::Lasers;
use crate::player::Player;

const SCORE_HEIGHT: f64 = 20.0;

#[derive(PartialEq)]
enum GameState {
    Starting,
    Running,
    Dead,
    Over
}

pub struct GameContext {
    controller: Controller,
    player_x: f64,
    player_y: f64,
    player_width: f64,
    player_height: f64
}

impl GameContext {
    fn new(controller: &mut Controller, player_x: f64, player_y: f64, player_width: f64, player_height: f64) -> Self {
        Self {
            controller: *controller,
            player_x,
            player_y,
            player_width,
            player_height
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

    pub fn get_player_width(&self) -> f64 {
        self.player_width
    }

    pub fn get_player_height(&self) -> f64 {
        self.player_height
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
    game_height: f64,
    lives: u32,
    score: u64,
    high_score: u64,
}

impl<'a> Game<'a> {
    pub fn new(
        window_width: f64, window_height: f64, font: &'a mut EmbeddedFile, hero_png: &mut EmbeddedFile, laser_png: &mut EmbeddedFile, enemy_png: &mut EmbeddedFile
    ) -> Self {
        let image = image::load_from_memory(hero_png.data.as_ref()).unwrap();
        let texture = Texture::from_image(image.as_rgba8().unwrap(), &TextureSettings::new());
        let size = texture.get_size();
        let game_height = window_height - SCORE_HEIGHT - size.1 as f64;
        Self {
            controller: Controller::new(window_width, game_height),
            glyphs: GlyphCache::from_bytes(font.data.as_ref(), (), TextureSettings::new()).unwrap(),
            lasers: Lasers::new(window_width, game_height, laser_png),
            player: Player::new(window_width, game_height, hero_png),
            enemies: Enemies::new(window_width, game_height, enemy_png, 3),
            state: GameState::Starting,
            window_width,
            window_height,
            game_height,
            lives: 3,
            score: 0,
            high_score: 0,
        }
    }

    fn display_status(&mut self, ctx: Context, gl: &mut GlGraphics) {
        let y = self.window_height - 30.0;
        let mut transform = ctx.transform.trans(48.0, y);
        text::Text::new_color(color::BLUE, 24).draw(
            &format!("{}", self.score), &mut self.glyphs, &ctx.draw_state, transform, gl
        ).unwrap();

        transform = ctx.transform.trans((self.window_width / 2.0) - (24.0 * 4.0), y);
        text::Text::new_color(color::BLUE, 24).draw(
            &format!("High: {}", self.high_score), &mut self.glyphs, &ctx.draw_state, transform, gl
        ).unwrap();

        transform = ctx.transform.trans(self.window_width - 72.0, y);
        text::Text::new_color(color::BLUE, 24).draw(
            &format!("{}", self.lives), &mut self.glyphs, &ctx.draw_state, transform, gl
        ).unwrap();
    }

    pub fn update(&mut self, event: &mut Event) {
        if let Some(args) = event.controller_axis_args() {
            self.controller.update(args);
        }
        if let Some(button) = event.release_args() {
            match button {
                Button::Controller(_) => {
                    if self.state == GameState::Over {
                        self.lives = 3;
                        self.score = 0;
                    }
                    self.state = GameState::Running
                },
                _ => {},
            }
        }

        if self.state == GameState::Running {
            let mut game_context = &GameContext::new(
                &mut self.controller,
                self.player.get_x(),
                self.player.get_y(),
                self.player.get_width(),
                self.player.get_height()
            );
            game_context = self.enemies.update(game_context);
            game_context = self.player.update(game_context);
            self.lasers.update(game_context);

            let mut enemies_to_remove: Vec<Uuid> = vec![];
            let mut lasers_to_remove: Vec<Uuid> = vec![];
            for (ei, e) in self.enemies.get_enemies().iter() {
                let er = e.get_rect();
                let pr = self.player.get_rect();
                if er.has_intersection(pr) {
                    self.lives = self.lives - 1;
                    enemies_to_remove.push(*ei);
                    self.state = GameState::Dead;
                } else {
                    for (li, l) in self.lasers.get_lasers().iter() {
                        let lr = l.get_rect();
                        if lr.has_intersection(er) {
                            self.score = self.score + 10;
                            self.high_score = self.high_score.max(self.score);
                            enemies_to_remove.push(*ei);
                            lasers_to_remove.push(*li);
                        }
                    }
                }
            }

            for e in enemies_to_remove.iter() {
                self.enemies.remove(e);
            }

            for l in lasers_to_remove.iter() {
                self.lasers.remove(l);
            }
        }
    }

    pub fn draw(&mut self, ctx: Context, gl: &mut GlGraphics) {
        self.display_status(ctx, gl);

        match self.state {
            GameState::Starting => {
                let transform = ctx.transform.trans((self.window_width / 2.0) - 250.0, (self.game_height / 2.0) - 14.0);
                text::Text::new_color(color::BLUE, 14).draw(
                    "Press a button to start",
                    &mut self.glyphs, &ctx.draw_state, transform, gl
                ).unwrap();
            }
            GameState::Dead => {
                if self.lives == 0 {
                    self.state = GameState::Over;
                } else {
                    let transform = ctx.transform.trans((self.window_width / 2.0) - 300.0, (self.game_height / 2.0) - 14.0);
                    text::Text::new_color(color::BLUE, 14).draw(
                        "Got you. Press any key to continue",
                        &mut self.glyphs, &ctx.draw_state, transform, gl
                    ).unwrap();
                }
            }
            GameState::Over => {
                let transform = ctx.transform.trans((self.window_width / 2.0) - 300.0, (self.game_height / 2.0) - 14.0);
                text::Text::new_color(color::BLUE, 14).draw(
                    "Game Over. Press any key to play again",
                    &mut self.glyphs, &ctx.draw_state, transform, gl
                ).unwrap();

                self.enemies.reset();
                self.lasers.reset();
                self.player.reset();
            }
            _ => {
                self.player.draw(ctx, gl);
                self.lasers.draw(ctx, gl);
                self.enemies.draw(ctx, gl);
            }
        }
    }
}
