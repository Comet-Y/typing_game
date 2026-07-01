use bevy::prelude::*;
use crate::assets;
use crate::ui::root;
use crate::ui::view_model;
use crate::state;
// #[derive(Resource)]
// struct KpmUi{
//     kpm_ui:Entity,
// }
pub fn enter_kpm_display(
    mut commands:Commands,
    parent:Single<Entity,With<root::TextParent>>,
    font_assets:Res<assets::fonts::FontAssets>,
    kpm_view_model:Res<view_model::KpmViewModel>
){
    let kpm_ui=spawn_kpm(&mut commands,&font_assets,&kpm_view_model);
    commands.entity(parent.entity()).add_child(kpm_ui);

}

fn spawn_kpm(
    commands:&mut Commands,
    font_assets:&Res<assets::fonts::FontAssets>,
    kpm_view_model:&Res<view_model::KpmViewModel>,
)->Entity{
    commands.spawn(
        (
            Text::new(format!("{}kpm",kpm_view_model.kpm)),
            font_assets.font_preset.clone(),
            DespawnOnExit(state::InGameState::DisplayKpm),
        )
    ).id()
}