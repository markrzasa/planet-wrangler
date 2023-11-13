use crate::black_hole::BlackHoles;
use crate::enemy::Enemies;
use crate::laser::Lasers;
use crate::planets::Planets;
use crate::player::Player;

pub struct GameElements {
    pub black_holes: BlackHoles,
    pub enemies: Enemies,
    pub lasers: Lasers,
    pub planets: Planets,
    pub player: Player
}
