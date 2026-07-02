use bevy::prelude::*;
#[derive(States, Clone, Debug, Hash, Eq, PartialEq, Default)]
#[states(scoped_entities)]
pub enum GameState {
    #[default]
    Start,
    MainMenu,
    InGame,
    EndMenu,
}

#[derive(SubStates, Clone, Debug, Hash, Eq, PartialEq, Default)]
#[source(GameState=GameState::InGame)]
pub enum InGameState{
    #[default]
    CountDown,
    Typing,
    DisplayKpm,
}