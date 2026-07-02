use bevy::prelude::*;
#[derive(Resource, Clone)]
pub struct SoundAssets {
    pub correct: Handle<AudioSource>,
    pub miss: Handle<AudioSource>,
    pub countdown_321:Handle<AudioSource>,
    pub countdown_0:Handle<AudioSource>
}
impl SoundAssets{
   pub fn load(asset_server:&AssetServer)->Self{
        Self{
            correct:asset_server.load("sounds/correct.mp3"),
            miss:asset_server.load("sounds/miss.mp3"),
            countdown_321:asset_server.load("sounds/countdown_321.mp3"),
            countdown_0:asset_server.load("sounds/countdown_0.mp3"),
        }
    }
}