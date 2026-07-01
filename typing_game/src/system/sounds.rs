use bevy::prelude::*;
use crate::assets::audio;
use crate::system::messages;
pub fn correct(
    commands:&mut Commands,
    sound_assets:&Res<audio::SoundAssets>
){
    commands.spawn((
        AudioPlayer::new(sound_assets.correct.clone()),
        PlaybackSettings::DESPAWN,
    ));
}

pub fn miss(
    commands:&mut Commands,
    sound_assets:&Res<audio::SoundAssets>
){
    commands.spawn((
        AudioPlayer::new(sound_assets.miss.clone()),
        PlaybackSettings::DESPAWN
    ));
}

pub fn handle_key_input_sound(
    mut key_input_result:MessageReader<messages::KeyInputResult>,
    mut commands:Commands,
    sound_asset:Res<audio::SoundAssets>
){
    for result in key_input_result.read(){
        match result.correct{
            true=>{
                correct(&mut commands,&sound_asset);
            },
            false=>{
                miss(&mut commands,&sound_asset);
            }
        }
    }
}