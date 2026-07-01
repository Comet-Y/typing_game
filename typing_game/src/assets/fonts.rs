use bevy::prelude::*;
#[derive(Resource)]
pub struct FontAssets{
    //pub japanese_font:Handle<Font>,
    pub font_preset:TextFont,
}

impl FontAssets{
    pub fn load(asset_server:&AssetServer)->Self{
        let japanese_font=asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");
        Self{
            //japanese_font:japanese_font.clone(),
            font_preset:TextFont{
                font:japanese_font.clone().into(),
                font_size:FontSize::Px(50.0),
                ..default()
            }
        }
    }
}
