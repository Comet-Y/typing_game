use bevy::prelude::*;
use crate::state;
use crate::system::timer;
use crate::system::messages;
#[derive(Resource)]
pub  struct CountDownRemain(pub u8);

pub fn start_countdown(
    mut commands:Commands,
    mut count_down_sound:MessageWriter<messages::CountDownSound>
){
    commands.insert_resource(CountDownRemain(3));
            count_down_sound.write(messages::CountDownSound{count:3});
}

pub fn reset_countdown_timer(
    mut count_down:ResMut<timer::CountDownTimer>,
    mut count_down_remain:ResMut<CountDownRemain>,
    mut count_down_sound:MessageWriter<messages::CountDownSound>
){
    if count_down.0.is_finished(){

        count_down_remain.0-=1;
        count_down_sound.write(messages::CountDownSound{count:count_down_remain.0});
        count_down.0.reset();
    }
}
pub fn finish_countdown(
    count_down_remain:Res<CountDownRemain>,
    mut state:ResMut<NextState<state::InGameState>>
){
    if count_down_remain.0==0{
        state.set(state::InGameState::Typing);
    }
}