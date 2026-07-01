use bevy::prelude::*;
use crate::typing::*;
use crate::state;
pub struct TypingPlugin;

impl Plugin for TypingPlugin{
    fn build(&self,app:&mut App){
        app.add_systems(OnEnter(state::GameState::Start),load_typingstate);
    }
}

fn load_typingstate(
    mut commands:Commands,
){
    println!("insert TypingState!");
    commands.insert_resource(romaji::RomajiDictionary::new());
    commands.insert_resource(session::TypingState::new());
}
