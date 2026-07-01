use bevy::prelude::*;
#[derive(Message)]
pub struct KeyInputResult{
    pub correct:bool,
}