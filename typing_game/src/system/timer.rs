use bevy::prelude::*;
use bevy::time::Stopwatch;
use crate::state;
use crate::typing::session;
pub struct GameTimer{
    pub stopwatch:Stopwatch,
    prev:f32,
}
#[derive(Resource)]
pub struct Timers{
    pub wait:Timer,
    pub game_timer:GameTimer
}

pub fn setup_timer(
    mut commands:Commands,
){
    let timers=Timers{
        wait:Timer::from_seconds(1.0,TimerMode::Once),
        game_timer:GameTimer{
            stopwatch:Stopwatch::new(),
            prev:0.0
        }
    };
    commands.insert_resource(timers);
}

pub fn tick_timers(
    mut timers:ResMut<Timers>,
    time:Res<Time>,
){
    timers.wait.tick(time.delta());
    timers.game_timer.stopwatch.tick(time.delta());
}

pub fn problem_elapsed(
    mut timers:ResMut<Timers>
)->f32{
    let problem_elapsed=timers.game_timer.stopwatch.elapsed_secs()-timers.game_timer.prev;
    timers.game_timer.prev=timers.game_timer.stopwatch.elapsed_secs();
    problem_elapsed
}

pub fn end_kpmdisplay(
    mut timers:ResMut<Timers>,
    mut next_gamestate:ResMut<NextState<state::GameState>>,
    mut next_ingamestate:ResMut<NextState<state::InGameState>>,
    typing_state:Res<session::TypingState>
){
    if timers.wait.is_finished(){
        if typing_state.progress.is_clear(){
            next_gamestate.set(state::GameState::EndMenu);
        }
        else{
            timers.game_timer.stopwatch.unpause();
            next_ingamestate.set(state::InGameState::Typing);
        }
    }
}