use bevy::prelude::*;
use crate::ui::*;
use crate::state;
use crate::schedule;

pub struct UiPlugin;

impl Plugin for UiPlugin{
    fn build(&self,app:&mut App){
        app.add_systems(OnEnter(state::GameState::Start),root::setup_ui_root.in_set(schedule::GameSet::Ui));
        app.add_systems(OnEnter(state::GameState::MainMenu),main_menu_ui::enter_main_menu.in_set(schedule::GameSet::Ui));
        app.add_systems(OnEnter(state::InGameState::CountDown),count_down::enter_countdown.in_set(schedule::GameSet::Ui));
        app.add_systems(OnExit(state::InGameState::CountDown),in_game_ui::enter_game.in_set(schedule::GameSet::Ui));
        app.add_systems(OnEnter(state::InGameState::Typing),(typing_ui::enter_typing,in_game_ui::update_problem_index).in_set(schedule::GameSet::Ui));
        app.add_systems(OnEnter(state::InGameState::DisplayKpm),kpm_display_ui::enter_kpm_display.in_set(schedule::GameSet::Ui));
        app.add_systems(OnEnter(state::GameState::EndMenu),end_menu_ui::enter_end_menu.in_set(schedule::GameSet::Ui));
        app.add_systems(FixedUpdate,count_down::change_countdown_ui.run_if(in_state(state::InGameState::CountDown)).in_set(schedule::GameSet::Ui));
        app.add_systems(FixedUpdate,(typing_ui::change_odai_kana_ui,typing_ui::change_input_ui).in_set(schedule::GameSet::Ui).run_if(in_state(state::InGameState::Typing)));
        app.insert_resource(view_model::InGameViewModel{
            problem_index:0,
            problem_count:0,});
        app.insert_resource(view_model::CountDownViewModel{
            count_down:0
        });
        app.insert_resource(view_model::ProblemViewModel{
            odai:String::new(),
            odai_kana:Vec::new(),
            kana_index:0,
            input:String::new(),
        });
        app.insert_resource(view_model::KpmViewModel{
            kpm:String::new(),
        });
        app.insert_resource(view_model::EndMenuViewModel{
            kpm:String::new(),
        });
    }
}