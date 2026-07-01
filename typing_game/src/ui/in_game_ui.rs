use bevy::prelude::*;
use crate::ui::root;
use crate::assets::fonts;
use crate::state;
use crate::ui::view_model;
#[derive(Resource)]
pub struct InGameUi{
    problem_index_ui:Entity,
}
pub fn enter_game(
    mut commands:Commands,
    parent:Single<Entity,With<root::TextParent>>,
    font_assets:Res<fonts::FontAssets>,
    in_game_view_model:Res<view_model::InGameViewModel>
){
    let problem_index_ui=spawn_problem_index(&mut commands,&font_assets,&in_game_view_model);
    commands.entity(parent.entity()).add_child(problem_index_ui);
    
    commands.insert_resource(
        InGameUi{
            problem_index_ui
        }
    )
}

fn spawn_problem_index(
    commands:&mut Commands,
    font_assets:&Res<fonts::FontAssets>,
    in_game_view_model:&Res<view_model::InGameViewModel>
)->Entity{
    commands.spawn((
        Text::new(format!("{}/{}",in_game_view_model.problem_index,in_game_view_model.problem_count)),
        font_assets.font_preset.clone(),
        Node{
            position_type:PositionType::Absolute,
            top:Val::Px(10.0),
            left:Val::Px(10.0),
            ..default()
        },
        DespawnOnExit(state::GameState::InGame)
    )).id()
}

pub fn update_problem_index(
    in_game_ui:Res<InGameUi>,
    in_game_view_model:Res<view_model::InGameViewModel>,
    mut query:Query<&mut Text>,
){
    if let Ok(mut index)=query.get_mut(in_game_ui.problem_index_ui){
        index.0=format!("{}/{}",in_game_view_model.problem_index,in_game_view_model.problem_count);
    }
}
