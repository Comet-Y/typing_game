use bevy::prelude::*;
use crate::assets::*;
pub struct AssetPlugin;
impl Plugin for AssetPlugin{
    fn build(&self,app:&mut App){
        app.add_systems(Startup,load_assets);
    }
}
fn load_assets(mut commands:Commands,asset_server:Res<AssetServer>){
    commands.insert_resource(audio::SoundAssets::load(&asset_server));
    commands.insert_resource(fonts::FontAssets::load(&asset_server));
}