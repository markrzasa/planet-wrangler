mod controller;
mod laser;
mod player;
mod enemy;
mod drawable;
mod updateable;
mod black_hole;
mod game_context;
mod planets;
mod game_sprite;

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
use rand::Rng;
use rust_embed::RustEmbed;
use sdl2::rect::Rect;
use sdl2_window::Sdl2Window;
use uuid::Uuid;
use crate::black_hole::{BlackHole, BlackHoles, BlackHoleState};
use crate::controller::Controller;
use crate::drawable::Drawable;
use crate::enemy::{Enemies, EnemyState};
use crate::game_context::GameContext;
use crate::laser::Lasers;
use crate::planets::{Planets, PlanetState};
use crate::player::{Player, PlayerState};
use crate::updateable::Updateable;

const SCORE_HEIGHT: f64 = 20.0;
const WINDOW_HEIGHT: f64 = 1000.0;
const WINDOW_WIDTH: f64 = 1000.0;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

#[derive(PartialEq)]
enum GameState {
    Starting,
    Running,
    LevelComplete,
    Dead,
    Over
}

fn new_rect(sprite_width: u32, sprite_height: u32, window_width: f64, window_height: f64) -> Rect {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0, window_width as u32);
    let y = rng.gen_range(0, window_height as u32);

    Rect::new(x as i32, y as i32, sprite_width, sprite_height)
}

fn get_spawn_points(sprite_width: u32, sprite_height: u32, window_width: f64, window_height: f64, no_spawn_rect: Rect, num_points: u32) -> Vec<Rect> {
    let mut points: Vec<Rect> = Vec::new();
    for _ in 0..num_points {
        let mut r = new_rect(sprite_width, sprite_height, window_width, window_height);
        while r.has_intersection(no_spawn_rect) {
            r = new_rect(sprite_width, sprite_height, window_width, window_height);
        }
        points.push(new_rect(sprite_width, sprite_height, window_width, window_height));
    }

    points
}

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

    let mut controller = Controller::new(window_width, window_height);
    let mut glyphs = GlyphCache::from_bytes(font.data.as_ref(), (), TextureSettings::new()).unwrap();
    let mut lasers = Lasers::new(window_width, game_height, &Assets::get("laser.png").unwrap());
    let mut player = Player::new(window_width, game_height, &hero_png);
    let mut enemies = Enemies::new(&Assets::get("enemy.png").unwrap(), 3);
    let mut black_holes = BlackHoles::new(&Assets::get("black-hole.png").unwrap());
    let mut planets = Planets::new(
        &Assets::get("done.png").unwrap(),
        &Assets::get("planets.png").unwrap(),
        3, window_width, game_height
    );
    let mut state = GameState::Starting;
    let mut lives = 3;
    let mut score = 0;
    let mut high_score = 0;

    let player_sprite = player.get_sprite();
    let x = (player_sprite.x - player_sprite.width) as i32;
    let y = (player_sprite.y - player_sprite.height) as i32;
    let w = (player_sprite.x + player_sprite.width) as u32;
    let h = (player_sprite.y + player_sprite.height) as u32;

    let no_spawn_rect = Rect::new(x, y, w, h);
    let mut num_spawn_points = 3;
    let mut spawn_points: Vec<Rect> = Vec::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.controller_axis_args() {
            controller.update(args);
        }
        if let Some(Button::Controller(_)) = event.release_args() {
            if state == GameState::Over {
                lives = 3;
                num_spawn_points = 3;
                score = 0;
            } else if state == GameState::LevelComplete {
                num_spawn_points += 1;
            }

            player.reset();
            for (_, planet) in planets.get_planets().iter_mut() {
                if planet.get_state() == PlanetState::Towed {
                    planet.not_towed();
                }
            }
            state = GameState::Running;
        }

        match state {
            GameState::Starting | GameState::Over | GameState::LevelComplete => {
                spawn_points.clear();
                black_holes.reset();
                planets.reset();
                enemies.reset();
                lasers.reset();
                player.reset();
            }
            GameState::Dead => {
                enemies.reset();
            }
            GameState::Running => {
                if spawn_points.is_empty() {
                    spawn_points = get_spawn_points(
                        player.get_sprite().width as u32, player.get_sprite().height as u32,
                        window_width, game_height,
                        no_spawn_rect, num_spawn_points
                    );
                    black_holes.set_black_holes(&spawn_points);
                }

                let mut game_context = & GameContext::new(
                    &mut controller,
                    &mut player,
                    game_height,
                    window_width,
                    black_holes.get_black_holes().iter()
                        .filter(|h|h.get_state() == BlackHoleState::Open)
                        .map(|h|h.get_sprite().get_position()).collect()
                );

                game_context = black_holes.update(game_context);
                game_context = planets.update(game_context);
                game_context = enemies.update(game_context);
                game_context = player.update(game_context);
                lasers.update(game_context);

                let pr = player.get_rect();
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
                        lives -= 1;
                        if lives == 0 {
                            state = GameState::Over;
                        } else {
                            state = GameState::Dead;
                        }
                        continue;
                    }

                    for (li, l) in lasers.get_lasers().iter() {
                        let lr = l.get_sprite().get_position();
                        if lr.has_intersection(er) {
                            e.dying();
                            lasers_to_remove.push( * li);
                            (score, high_score) = update_score(score, high_score, 10);
                        }
                    }
                }

                if player.get_state() == PlayerState::NotTowing {
                    for (_, planet) in planets.get_planets().iter_mut() {
                        if (planet.get_state() == PlanetState::NotTowed) && (planet.get_sprite().get_position().has_intersection(pr)) {
                            player.towing();
                            planet.towed();
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
                                (score, high_score) = update_score(score, high_score, 100);
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
                        state = GameState::LevelComplete;
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
                    &format!("{}", score), &mut glyphs, &ctx.draw_state, transform, gl
                ).unwrap();

                transform = ctx.transform.trans((window_width / 2.0) - (24.0 * 4.0), y);
                text::Text::new_color(color::YELLOW, 24).draw(
                    &format!("High: {}", high_score), &mut glyphs, &ctx.draw_state, transform, gl
                ).unwrap();

                transform = ctx.transform.trans(window_width - 72.0, y);
                text::Text::new_color(color::YELLOW, 24).draw(
                    &format!("{}", lives), &mut glyphs, &ctx.draw_state, transform, gl
                ).unwrap();

                match state {
                    GameState::Starting | GameState::Over => {
                        if state == GameState::Starting {
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
