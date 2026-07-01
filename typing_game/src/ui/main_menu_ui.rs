use bevy::prelude::*;
use crate::assets;
use crate::ui::root;
use crate::state;

// #[derive(Resource)]
// struct MainMenuUi{
//     prompt_ui:Entity,
// }

pub fn enter_main_menu(
    mut commands:Commands,
    parent:Single<Entity,With<root::TextParent>>,
    font_assets:Res<assets::fonts::FontAssets>,
){
    let prompt_ui=spawn_prompt(&mut commands,&font_assets);
    commands.entity(parent.entity()).add_child(prompt_ui);
}

fn spawn_prompt(
    commands:&mut Commands,
    font_assets:&Res<assets::fonts::FontAssets>
)->Entity{
    commands.spawn(
        (
            Text::new("press space to start"),
            font_assets.font_preset.clone(),
            DespawnOnExit(state::GameState::MainMenu),
        )
    ).id()
}