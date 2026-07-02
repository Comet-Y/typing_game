use bevy::prelude::*;
use crate::system::*;
use crate::schedule;
use crate::state;

pub struct SystemPlugin;

impl Plugin for SystemPlugin{
    fn build(
        &self,
        app:&mut App,
    ){
        app.add_message::<messages::KeyInputResult>();
        app.configure_sets(FixedUpdate,(
            schedule::GameSet::System,
            schedule::GameSet::ViewModel.after(schedule::GameSet::System),
            schedule::GameSet::Ui.after(schedule::GameSet::ViewModel),
        ));
            app.configure_sets(OnEnter(state::GameState::MainMenu),(
            schedule::GameSet::System,
            schedule::GameSet::ViewModel.after(schedule::GameSet::System),
            schedule::GameSet::Ui.after(schedule::GameSet::ViewModel),
        ));

          app.configure_sets(OnEnter(state::InGameState::CountDown),(
            schedule::GameSet::System,
            schedule::GameSet::ViewModel.after(schedule::GameSet::System),
            schedule::GameSet::Ui.after(schedule::GameSet::ViewModel),
        ));
           app.configure_sets(OnEnter(state::GameState::InGame),(
            schedule::GameSet::System,
            schedule::GameSet::ViewModel.after(schedule::GameSet::System),
            schedule::GameSet::Ui.after(schedule::GameSet::ViewModel),
        ));
            app.configure_sets(OnEnter(state::GameState::EndMenu),(
            schedule::GameSet::System,
            schedule::GameSet::ViewModel.after(schedule::GameSet::System),
            schedule::GameSet::Ui.after(schedule::GameSet::ViewModel),
        ));
    
         app.configure_sets(OnEnter(state::InGameState::Typing),(
            schedule::GameSet::System,
            schedule::GameSet::ViewModel.after(schedule::GameSet::System),
            schedule::GameSet::Ui.after(schedule::GameSet::ViewModel),
        ));
         app.configure_sets(OnEnter(state::InGameState::DisplayKpm),(
            schedule::GameSet::System,
            schedule::GameSet::ViewModel.after(schedule::GameSet::System),
            schedule::GameSet::Ui.after(schedule::GameSet::ViewModel),
        ));
        app.add_systems(Startup,(build_problems::build_problems,on_enter::start).in_set(schedule::GameSet::System));
        app.add_systems(OnEnter(state::GameState::MainMenu),timer::setup_timer);
        app.add_systems(OnEnter(state::InGameState::CountDown),(timer::start_countdown,countdown::start_countdown).in_set(schedule::GameSet::Ui));

        app.add_systems(OnExit(state::InGameState::CountDown),build_viewmodel::build_ingame_viewmodel.in_set(schedule::GameSet::ViewModel));
        app.add_systems(OnEnter(state::InGameState::Typing),(timer::start_stopwatch,build_viewmodel::build_problem_viewmodel,build_viewmodel::update_ingame_viewmodel).in_set(schedule::GameSet::ViewModel));
        app.add_systems(OnEnter(state::InGameState::DisplayKpm),build_viewmodel::build_kpm_viewmodel.in_set(schedule::GameSet::ViewModel));
        app.add_systems(OnEnter(state::GameState::EndMenu),build_viewmodel::build_endmenu_viewmodel.in_set(schedule::GameSet::ViewModel));
        app.add_systems(FixedUpdate,handle_key_input::handle_key_input_mainmenu.run_if(in_state(state::GameState::MainMenu)).in_set(schedule::GameSet::System));
        app.add_systems(FixedUpdate,handle_key_input::handle_key_input_ingame.run_if(in_state(state::InGameState::Typing)).in_set(schedule::GameSet::System));
        app.add_systems(FixedUpdate,handle_key_input::handle_key_input_endmenu.run_if(in_state(state::GameState::EndMenu)).in_set(schedule::GameSet::System));
        app.add_systems(FixedUpdate,timer::end_kpmdisplay.run_if(in_state(state::InGameState::DisplayKpm)).in_set(schedule::GameSet::System));
        app.add_systems(FixedUpdate,sounds::handle_key_input_sound.run_if(in_state(state::InGameState::Typing)).in_set(schedule::GameSet::System));
        app.add_systems(FixedUpdate,build_viewmodel::build_countdown_viewmodel.run_if(in_state(state::InGameState::CountDown)).in_set(schedule::GameSet::ViewModel));
        app.add_systems(FixedUpdate,build_viewmodel::update_problem_viewmodel.run_if(in_state(state::InGameState::Typing)).in_set(schedule::GameSet::ViewModel));
        app.add_systems(FixedUpdate,(countdown::finish_countdown,countdown::reset_countdown_timer).run_if(in_state(state::InGameState::CountDown)).in_set(schedule::GameSet::System));
        app.add_systems(FixedUpdate,handle_key_input::handle_key_input_result.run_if(in_state(state::InGameState::Typing)).in_set(schedule::GameSet::System));
        app.add_systems(FixedUpdate,timer::tick_timers.run_if(in_state(state::GameState::InGame)).in_set(schedule::GameSet::System));
    }
}