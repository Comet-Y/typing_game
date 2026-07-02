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
#[derive(Resource)]
pub struct CountDownTimer(pub Timer);

pub fn setup_timer(
    mut commands:Commands,
){
    let mut timers=Timers{
        wait:Timer::from_seconds(1.0,TimerMode::Once),
        game_timer:GameTimer{
            stopwatch:Stopwatch::new(),
            prev:0.0
        }
    };
    timers.wait.pause();
    timers.game_timer.stopwatch.pause();
    let mut count_down=CountDownTimer(Timer::from_seconds(1.0,TimerMode::Once));
    count_down.0.pause();
    commands.insert_resource(count_down);
    commands.insert_resource(timers);
}

pub fn tick_timers(
    mut timers:ResMut<Timers>,
    mut count_down:ResMut<CountDownTimer>,
    time:Res<Time>,
){
    count_down.0.tick(time.delta()); 
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
    timers:Res<Timers>,
    mut next_gamestate:ResMut<NextState<state::GameState>>,
    mut next_ingamestate:ResMut<NextState<state::InGameState>>,
    typing_state:Res<session::TypingState>
){
    if timers.wait.is_finished(){
        if typing_state.progress.is_clear(){
            next_gamestate.set(state::GameState::EndMenu);
        }
        else{
            next_ingamestate.set(state::InGameState::Typing);
        }
    }
}


pub fn start_countdown(
    mut count_down:ResMut<CountDownTimer>
){
        count_down.0.unpause();
}
pub fn start_stopwatch(
        mut timers:ResMut<Timers>,
){
            timers.game_timer.stopwatch.unpause();
}