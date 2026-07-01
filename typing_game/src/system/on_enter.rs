use bevy::prelude::*;
use crate::state;
pub fn start(
    mut next_gamestate:ResMut<NextState<state::GameState>>
){
    next_gamestate.set(state::GameState::MainMenu);
}
