use bevy::prelude::*;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key,KeyboardInput};
use crate::typing::session;
use crate::typing::romaji;
use crate::system::{messages,timer};
use crate::state;


pub fn handle_key_input_mainmenu(
    mut msg_kbd:MessageReader<KeyboardInput>,
    mut next_gamestate:ResMut<NextState<state::GameState>>
){
    for kbd in msg_kbd.read(){
        if kbd.state==ButtonState::Released{
            continue;
        };
        if kbd.repeat{
            continue;
        }
        match kbd.logical_key{
            Key::Space=>{
                next_gamestate.set(state::GameState::InGame);
            },
            _=>{}
        }
    }
}

pub fn handle_key_input_ingame(
    mut msg_kbd:MessageReader<KeyboardInput>,
    mut typing_state:ResMut<session::TypingState>,
    dict:Res<romaji::RomajiDictionary>,
    mut key_input_result:MessageWriter<messages::KeyInputResult>
){
    for kbd in msg_kbd.read(){
        if kbd.state==ButtonState::Released{
            continue;
        };
        if kbd.repeat{
            continue;
        }
        if let Key::Character(keyinput)=&kbd.logical_key{
            for c in keyinput.chars(){
                match typing_state.advance(&dict,c){
                    Ok(())=>{
                        key_input_result.write(messages::KeyInputResult{correct:true});
                    },
                    Err(())=>{
                        key_input_result.write(messages::KeyInputResult{correct:false});
                    }
                }
        }
    }
}
}

pub fn handle_key_input_endmenu(
    mut msg_kbd:MessageReader<KeyboardInput>,
    mut next_gamestate:ResMut<NextState<state::GameState>>,
    mut exit:MessageWriter<AppExit>,
    mut typing_state:ResMut<session::TypingState>
){
    for kbd in msg_kbd.read(){
        if kbd.state==ButtonState::Released{
            continue;
        };
        if kbd.repeat{
            continue;
        }
        match kbd.logical_key{
            
            Key::Space=>{
                typing_state.return_to_start();
                next_gamestate.set(state::GameState::MainMenu);
            },
            Key::Enter=>{
                exit.write(AppExit::Success);
            },
            _=>{}
        }

    }
}



pub fn handle_key_input_result(
    mut key_input_result:MessageReader<messages::KeyInputResult>,
    mut next_ingamestate:ResMut<NextState<state::InGameState>>,
    mut typing_state:ResMut<session::TypingState>,
    mut timer:ResMut<timer::Timers>
){
    for result in key_input_result.read(){
        match result.correct{
            true=>{
                if typing_state.is_clear(){
                    timer.game_timer.stopwatch.pause();
                    timer.wait.reset();
                    timer.wait.unpause();
                    next_ingamestate.set(state::InGameState::DisplayKpm);
                    typing_state.goto_next_problem();
                }
            },
            false=>{
            }
        }
    }
}
