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
pub fn countdown_321(
    commands:&mut Commands,
    sound_assets:&Res<audio::SoundAssets>
){
    commands.spawn((
        AudioPlayer::new(sound_assets.countdown_321.clone()),
        PlaybackSettings::DESPAWN
    ));
}

pub fn countdown_0(
    commands:&mut Commands,
    sound_assets:&Res<audio::SoundAssets>
){
    commands.spawn((
        AudioPlayer::new(sound_assets.countdown_0.clone()),
        PlaybackSettings::DESPAWN
    ));
}

pub fn handle_key_input_sound(
    mut key_input_result:MessageReader<messages::KeyInputResult>,
    mut commands:Commands,
    sound_assets:Res<audio::SoundAssets>
){
    for result in key_input_result.read(){
        match result.correct{
            true=>{
                correct(&mut commands,&sound_assets);
            },
            false=>{
                miss(&mut commands,&sound_assets);
            }
        }
    }
}

pub fn handle_countdown_sound(
    mut count_down_sound:MessageReader<messages::CountDownSound>,
    mut commands:Commands,
    sound_assets:Res<audio::SoundAssets>
){
    for count in count_down_sound.read(){
        match count.count{
            1..=3=>{
                countdown_321(&mut commands,&sound_assets);
            },
            0=>{
                countdown_0(&mut commands,&sound_assets);
            },
            _=>{
                panic!("count over three");
            }
        }
    }
}