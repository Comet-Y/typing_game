use bevy::prelude::*;
use crate::assets;
use crate::ui::root;
use crate::ui::view_model;
use crate::state;

#[derive(Resource)]
pub struct CountDownUi{
    countdown_ui:Entity,
}
pub fn enter_countdown(
    mut commands:Commands, 
    font_assets:Res<assets::fonts::FontAssets>,
    parent:Single<Entity,With<root::TextParent>>
){
    let countdown_ui=spawn_countdown(&mut commands,&font_assets);
    commands.entity(parent.entity()).add_child(countdown_ui);
    commands.insert_resource(CountDownUi{countdown_ui});
}

fn spawn_countdown(
    commands:&mut Commands, 
    font_assets:&Res<assets::fonts::FontAssets>
)->Entity{
    commands.spawn((
        Text::new("3"),
        font_assets.font_preset.clone(),
        DespawnOnExit(state::InGameState::CountDown),
    )).id()
}

pub fn change_countdown_ui(
    countdown_ui:Res<CountDownUi>,
    mut query:Query<&mut Text>,
    countdown_viewmodel:Res<view_model::CountDownViewModel>
){
    if let Ok(mut countdown)=query.get_mut(countdown_ui.countdown_ui.entity()){
        countdown.0=countdown_viewmodel.count_down.to_string();
    }
}