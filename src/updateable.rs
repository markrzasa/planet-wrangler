use crate::game_context::GameContext;

pub trait Updateable {
    fn update<'a>(&'a mut self, context: &'a GameContext) -> &GameContext;
    fn reset(&mut self);
}
