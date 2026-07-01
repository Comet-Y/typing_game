use bevy::prelude::*;
#[derive(Resource, Clone)]
pub struct SoundAssets {
    pub correct: Handle<AudioSource>,
    pub miss: Handle<AudioSource>,
}
impl SoundAssets{
   pub fn load(asset_server:&AssetServer)->Self{
        Self{
            correct:asset_server.load("sounds/correct.mp3"),
            miss:asset_server.load("sounds/miss.mp3")
        }
    }
}