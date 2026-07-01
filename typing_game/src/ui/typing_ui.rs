use bevy::prelude::*;
use crate::assets;
use crate::ui::*;
use crate::state;
#[derive(Component)]
pub struct KanaSpan(usize);
#[derive(Resource)]
pub struct TypingUi{
//    odai_ui:Entity,
    odai_kana_ui:Entity,
    input_ui:Entity,
}

pub fn enter_typing(
    mut commands:Commands,
    parent:Single<Entity,With<root::TextParent>>,
    font_assets:Res<assets::fonts::FontAssets>,
    problem_view_model:Res<view_model::ProblemViewModel>
){
    let odai_ui=spawn_odai(&mut commands,&font_assets,&problem_view_model);
    let odai_kana_ui=spawn_odai_kana(&mut commands,&font_assets,&problem_view_model);
    let input_ui=spawn_input(&mut commands,&font_assets);
    commands.entity(parent.entity()).add_child(odai_ui);
    commands.entity(parent.entity()).add_child(odai_kana_ui);
    commands.entity(parent.entity()).add_child(input_ui);
    commands.insert_resource(
        TypingUi{
//            odai_ui,
            odai_kana_ui,
            input_ui,
        }
    )

}
fn spawn_odai(
    commands:&mut Commands,
    font_assets:&Res<assets::fonts::FontAssets>,
    problem_view_model:&Res<view_model::ProblemViewModel>
)->Entity{
commands.spawn((
        Text::new(&problem_view_model.odai),
        font_assets.font_preset.clone(),
        DespawnOnExit(state::InGameState::Typing),
    )).id()
}

fn spawn_odai_kana(
        commands:&mut Commands,
    font_assets:&Res<assets::fonts::FontAssets>,
    problem_view_model:&Res<view_model::ProblemViewModel>
)->Entity{
    commands.spawn((
        Text::new(""),
        font_assets.font_preset.clone(),
        DespawnOnExit(state::InGameState::Typing),
    )).with_children(|p|{
        for (i,kana) in problem_view_model.odai_kana.iter().enumerate(){
            p.spawn((
                TextSpan(kana.clone()),
                DespawnOnExit(state::InGameState::Typing),
                font_assets.font_preset.clone(),
                TextColor(Color::WHITE),
                KanaSpan(i),
            ));
        }
    }).id()
}
fn spawn_input(
    commands:&mut Commands,
    font_assets:&Res<assets::fonts::FontAssets>,
)->Entity{
    commands.spawn((
        Text::new(""),
        font_assets.font_preset.clone(),
        DespawnOnExit(state::InGameState::Typing),
    )).id()
}

pub fn change_odai_kana_ui(
    typing_ui:Res<TypingUi>,
    problem_view_model:Res<view_model::ProblemViewModel>,
    mut odai_kana_children:Query<&Children>,
    mut query:Query<(&mut TextColor,&KanaSpan)>
){
    if let Ok(children)=odai_kana_children.get_mut(typing_ui.odai_kana_ui){
        for child in children{
            if let Ok((mut color,span))=query.get_mut(*child){
                if span.0<problem_view_model.kana_index{
                    color.0=Color::srgb(0.36,0.53,0.66);
                }
            }
        }
    }
}

pub fn change_input_ui(
    typing_ui:Res<TypingUi>,
    problem_view_model:Res<view_model::ProblemViewModel>,
    mut query:Query<&mut Text>
){
    if let Ok(mut input)=query.get_mut(typing_ui.input_ui){
        input.0=problem_view_model.input.clone();
    }
}