use bevy::prelude::*;
#[derive(Component)]
pub struct TextParent;
pub fn setup_ui_root(
    mut commands: Commands,
){
    commands.spawn(Camera2d);
    commands.spawn(
        (
            Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
                TextParent,));
}