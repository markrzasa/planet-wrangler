use sdl2::rect::Rect;
use crate::controller::Controller;
use crate::player::Player;

pub struct GameContext {
    controller: Controller,
    player: Rect,
    screen_height: f64,
    screen_width: f64,
    spawn_points: Vec<Rect>
}

impl GameContext {
    pub fn new(
        controller: &mut Controller,
        player: &mut Player,
        screen_height: f64,
        screen_width: f64,
        spawn_points: Vec<Rect>
    ) -> Self {
        Self {
            controller: *controller,
            player: player.get_rect(),
            screen_height,
            screen_width,
            spawn_points
        }
    }

    pub fn get_controller(&self) -> Controller {
        self.controller
    }

    pub fn get_player(&self) -> Rect {
        self.player
    }

    pub fn get_spawn_points(&self) -> Vec<Rect> {
        self.spawn_points.to_vec()
    }

    pub fn get_screen_height(&self) -> f64 {
        self.screen_height
    }

    pub fn get_screen_width(&self) -> f64 {
        self.screen_width
    }
}

