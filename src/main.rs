mod controller;
mod laser;
mod player;
mod enemy;
mod black_hole;
mod planets;
mod game_sprite;
mod game;

extern crate graphics;
extern crate image;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;
extern crate sprite;
extern crate rust_embed;

use graphics::{clear, text, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, ImageSize, Texture};
use piston::{Button, ControllerAxisEvent, Events, EventSettings, RenderEvent, ReleaseEvent};
use piston::window::WindowSettings;
use piston_window::{color, TextureSettings, Window};
use rust_embed::RustEmbed;
use sdl2_window::Sdl2Window;
use uuid::Uuid;
use crate::black_hole::{BlackHole, BlackHoles, BlackHoleState};
use crate::controller::Controller;
use crate::enemy::{Enemies, EnemyState};
use crate::game::{Game, GameState};
use crate::laser::Lasers;
use crate::planets::{Planets, PlanetState};
use crate::player::{Player, PlayerState};

const SCORE_HEIGHT: f64 = 20.0;
const WINDOW_HEIGHT: f64 = 1000.0;
const WINDOW_WIDTH: f64 = 1000.0;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn update_score(score: u32, high_score: u32, increment: u32) -> (u32, u32) {
    let new_score = score + increment;
    let new_high_score = high_score.max(new_score);
    (new_score, new_high_score)
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window = WindowSettings::new("Planet Wrangler", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .fullscreen(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let font = Assets::get("PressStart2PRegular.ttf").unwrap();
    let hero_png = Assets::get("hero.png").unwrap();
    let image = image::load_from_memory(hero_png.data.as_ref()).unwrap();
    let texture = Texture::from_image(image.as_rgba8().unwrap(), &TextureSettings::new());
    let size = texture.get_size();

    let window_width = window.size().width;
    let window_height = window.size().height;
    let game_height = window_height - SCORE_HEIGHT - size.1 as f64;

    let mut glyphs = GlyphCache::from_bytes(font.data.as_ref(), (), TextureSettings::new()).unwrap();

    let mut black_holes = BlackHoles::new(&Assets::get("black-hole.png").unwrap());
    let mut enemies = Enemies::new(&Assets::get("enemy.png").unwrap(), 3);
    let mut lasers = Lasers::new(window_width, game_height, &Assets::get("laser.png").unwrap());
    let mut planets = Planets::new(
        &Assets::get("done.png").unwrap(),
        &Assets::get("planets.png").unwrap(),
        3, window_width, game_height
    );
    let mut player = Player::new(window_width, game_height, &hero_png);

    let mut game = Game{
        black_hole_count: 3,
        black_holes: Vec::new(),
        controller: Controller::new(window_width, window_height),
        high_score: 0,
        lives: 3,
        player: player.get_sprite().get_position(),
        score: 0,
        screen_height: game_height,
        screen_width: window_width,
        state: GameState::Starting
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.controller_axis_args() {
            game.controller.update(args);
        }

        if let Some(Button::Controller(_)) = event.release_args() {
            if game.state != GameState::Running {
                if game.state == GameState::Over {
                    game.lives = 3;
                    game.black_hole_count = 3;
                    game.score = 0;
                } else if game.state == GameState::LevelComplete {
                    game.black_hole_count += 1;
                }

                player.reset();
                for (_, planet) in planets.get_planets().iter_mut() {
                    if planet.get_state() == PlanetState::Towed {
                        planet.not_towed();
                    }
                }
                game.state = GameState::Running;
            }
        }

        match game.state {
            GameState::Starting | GameState::Over | GameState::LevelComplete => {
                black_holes.reset();
                enemies.reset();
                lasers.reset();
                planets.reset();
                player.reset();
            }
            GameState::Dying => {
                player.update(&game);
                if player.get_state() == PlayerState::Dead {
                    if game.lives == 0 {
                        game.state = GameState::Over;
                    } else {
                        game.state = GameState::Dead;
                    }
                }
            }
            GameState::Dead => {
                enemies.reset();
                lasers.reset();
            }
            GameState::Running => {
                game.player = player.get_sprite().get_position();
                game.black_holes = black_holes.get_black_holes().iter()
                    .filter(|h|h.get_state() == BlackHoleState::Open)
                    .map(|h|h.get_sprite().get_position()).collect();

                player.update(&game);
                black_holes.update(&game);
                planets.update(&game);
                enemies.update(&game);
                player.update(&game);
                lasers.update(&game);

                let pr = player.get_sprite().get_position();
                let mut enemies_to_remove: Vec<Uuid> = vec![];
                let mut lasers_to_remove: Vec<Uuid> = vec![];
                for (ei, e) in enemies.get_enemies().iter_mut() {
                    if e.get_state() == EnemyState::Dying {
                        continue;
                    }

                    if e.get_state() == EnemyState::Dead {
                        enemies_to_remove.push( *ei);
                        continue;
                    }

                    let er = e.get_sprite().get_position();
                    if er.has_intersection(pr) {
                        enemies_to_remove.push( *ei);
                        game.lives -= 1;
                        player.dying();
                        game.state = GameState::Dying;
                        continue;
                    }

                    for (li, l) in lasers.get_lasers().iter() {
                        let lr = l.get_sprite().get_position();
                        if lr.has_intersection(er) {
                            e.dying();
                            lasers_to_remove.push( * li);
                            (game.score, game.high_score) = update_score(game.score, game.high_score, 10);
                        }
                    }
                }

                if player.get_state() == PlayerState::NotTowing {
                    for (_, planet) in planets.get_planets().iter_mut() {
                        if (planet.get_state() == PlanetState::NotTowed) && (planet.get_sprite().get_position().has_intersection(pr)) {
                            player.towing();
                            planet.towed();
                            break;
                        }
                    }
                }

                for (_, planet) in planets.get_planets().iter_mut() {
                    if planet.get_state() == PlanetState::Towed {
                        for black_hole in black_holes.get_black_holes().iter_mut() {
                            if (black_hole.get_state() == BlackHoleState::Open) && (black_hole.get_sprite().get_position().has_intersection(planet.get_sprite().get_position())) {
                                black_hole.covered();
                                planet.in_place(black_hole.get_sprite().get_position());
                                player.not_towing();
                                (game.score, game.high_score) = update_score(game.score, game.high_score, 100);
                                break;
                            }
                        }
                    }
                }

                for e in enemies_to_remove.iter() {
                    enemies.remove(e);
                }

                for l in lasers_to_remove.iter() {
                    lasers.remove(l);
                }

                if !black_holes.get_black_holes().is_empty() {
                    let open_black_holes: Vec<&BlackHole> = black_holes.get_black_holes().iter().filter(|h|h.get_state() == BlackHoleState::Open).collect();
                    if open_black_holes.is_empty() {
                        game.state = GameState::LevelComplete;
                    }
                }
            }
        }

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |ctx, gl| {
                clear(color::BLACK, gl);
                let y = window_height - 30.0;
                let mut transform = ctx.transform.trans(48.0, y);
                text::Text::new_color(color::YELLOW, 24).draw(
                    &format!("{}", game.score), &mut glyphs, &ctx.draw_state, transform, gl
                ).unwrap();

                transform = ctx.transform.trans((window_width / 2.0) - (24.0 * 4.0), y);
                text::Text::new_color(color::YELLOW, 24).draw(
                    &format!("High: {}", game.high_score), &mut glyphs, &ctx.draw_state, transform, gl
                ).unwrap();

                transform = ctx.transform.trans(window_width - 72.0, y);
                text::Text::new_color(color::YELLOW, 24).draw(
                    &format!("{}", game.lives), &mut glyphs, &ctx.draw_state, transform, gl
                ).unwrap();

                match game.state {
                    GameState::Starting | GameState::Over => {
                        if game.state == GameState::Starting {
                            let transform = ctx.transform.trans((window_width / 2.0) - 250.0, (game_height / 2.0) - 14.0);
                            text::Text::new_color(color::YELLOW, 14).draw(
                                "Press a button to start",
                                &mut glyphs, &ctx.draw_state, transform, gl
                            ).unwrap();
                        } else {
                            let transform = ctx.transform.trans((window_width / 2.0) - 300.0, (game_height / 2.0) - 14.0);
                            text::Text::new_color(color::YELLOW, 14).draw(
                                "Game Over. Press any key to play again",
                                &mut glyphs, &ctx.draw_state, transform, gl
                            ).unwrap();
                        }
                    }
                    GameState::Dead => {
                        let transform = ctx.transform.trans((window_width / 2.0) - 300.0, (game_height / 2.0) - 14.0);
                        text::Text::new_color(color::YELLOW, 14).draw(
                            "Got you. Press any key to continue",
                            &mut glyphs, &ctx.draw_state, transform, gl
                        ).unwrap();
                    }
                    GameState::LevelComplete => {
                        let transform = ctx.transform.trans((window_width / 2.0) - 370.0, (game_height / 2.0) - 14.0);
                        text::Text::new_color(color::YELLOW, 14).draw(
                            "Level complete. Press any key to continue",
                            &mut glyphs, &ctx.draw_state, transform, gl
                        ).unwrap();
                    }
                    _ => {
                        black_holes.draw(ctx, gl);
                        planets.draw(ctx, gl);
                        player.draw(ctx, gl);
                        lasers.draw(ctx, gl);
                        enemies.draw(ctx, gl);
                    }
                }
            });
        }
    }
}
