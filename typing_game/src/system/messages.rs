use bevy::prelude::*;
#[derive(Message)]
pub struct KeyInputResult{
    pub correct:bool,
}

#[derive(Message)]
pub struct CountDownSound{
    pub count:u8,
}