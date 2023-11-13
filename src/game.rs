use sdl2::rect::Rect;
use crate::controller::Controller;

#[derive(PartialEq)]
pub enum GameState {
    Starting,
    Running,
    LevelComplete,
    Dying,
    Dead,
    Over
}

pub struct Game {
    pub black_hole_count: u32,
    pub black_holes: Vec<Rect>,
    pub controller: Controller,
    pub high_score: u32,
    pub lives: u32,
    pub player: Rect,
    pub score: u32,
    pub screen_height: f64,
    pub screen_width: f64,
    pub state: GameState
}
