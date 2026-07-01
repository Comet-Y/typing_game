use bevy::prelude::*;
use crate::assets;
use crate::ui::root;
use crate::ui::view_model;
use crate::state;
// #[derive(Resource)]
// struct EndMenuUi{
//     kpm_ui:Entity,
// }
pub fn enter_end_menu(
    mut commands:Commands,
    parent:Single<Entity,With<root::TextParent>>,
    font_assets:Res<assets::fonts::FontAssets>,
    end_menu_view_model:Res<view_model::EndMenuViewModel>
){
    let kpm_ui=spawn_kpm(&mut commands,&font_assets,&end_menu_view_model);
    commands.entity(parent.entity()).add_child(kpm_ui);

}

fn spawn_kpm(
    commands:&mut Commands,
    font_assets:&Res<assets::fonts::FontAssets>,
    end_menu_view_model:&Res<view_model::EndMenuViewModel>,
)->Entity{
    commands.spawn(
        (
            Text::new(format!("{}kpm\npress space to retry.\npress enter to close window.",end_menu_view_model.kpm.to_string())),
            font_assets.font_preset.clone(),
            DespawnOnExit(state::GameState::EndMenu),
        )
    ).id()
}