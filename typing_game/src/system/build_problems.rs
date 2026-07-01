use bevy::prelude::*;
use crate::typing::session;
use crate::typing::romaji;
use std::io::prelude::*;
use crate::state;
use serde_json;
pub fn build_problems(
    mut typing_state:ResMut<session::TypingState>,
    dict:Res<romaji::RomajiDictionary>,
    state:Res<State<state::GameState>>
){
    println!("build problems!");
    let mut file=match std::fs::File::open("problems.json"){
    Ok(file)=>{
       file
    }
    Err(why)=>panic!("couldn't open problems.json:{}",why)
    };

    let mut s=String::new();
    file.read_to_string(&mut s).expect("couldn't read!");
    let odais:Vec<Vec<String>>=serde_json::from_str(&s).unwrap();
    for odai in odais{
        typing_state.add_odai(&odai[0],&odai[1],&dict);
    }
    println!("{:?}",state.get());
}