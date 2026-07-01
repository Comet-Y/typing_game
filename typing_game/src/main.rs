use bevy::prelude::*;
mod assets;
mod system;
mod typing;
mod ui;
mod schedule;
mod state;
fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<state::GameState>()
        .add_sub_state::<state::InGameState>()
        .add_plugins((
            typing::plugin::TypingPlugin,
            assets::plugin::AssetPlugin,
            system::plugin::SystemPlugin,
            ui::plugin::UiPlugin,
        )
        ).run();
}