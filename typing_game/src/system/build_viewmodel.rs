use bevy::prelude::*;
use crate::typing::session;
use crate::ui::view_model;
use crate::system::timer;
pub fn build_ingame_viewmodel(
    typing_state:Res<session::TypingState>,
    mut ingame_viewmodel:ResMut<view_model::InGameViewModel>
){
    ingame_viewmodel.problem_index=typing_state.progress.problem_index;
    ingame_viewmodel.problem_count=typing_state.progress.problem_count;
}

pub fn build_problem_viewmodel(
    typing_state:Res<session::TypingState>,
    mut problem_viewmodel:ResMut<view_model::ProblemViewModel>
){
    problem_viewmodel.odai=typing_state.get_current_problem().odai.clone();
    problem_viewmodel.odai_kana=typing_state.get_current_problem().kana_units.iter().map(|kana_unit|{
        kana_unit.kana_chunk.clone()
    }).collect();
    problem_viewmodel.kana_index=typing_state.current_problem_session.kana_index;
    problem_viewmodel.input=typing_state.current_problem_session.inputbuf.clone();
}

pub fn build_kpm_viewmodel(
    typing_state:Res<session::TypingState>,
    mut kpm_viewmodel:ResMut<view_model::KpmViewModel>,
    timer:ResMut<timer::Timers>
){
    kpm_viewmodel.kpm=60.0*typing_state.kpm_data.last_inputbuf_len as f32/timer::problem_elapsed(timer);
}

pub fn build_endmenu_viewmodel(
    typing_state:Res<session::TypingState>,
    mut endmenu_viewmodel:ResMut<view_model::EndMenuViewModel>,
    timer:Res<timer::Timers>
){
    endmenu_viewmodel.kpm=60.0*typing_state.kpm_data.typed_sum as f32/timer.game_timer.stopwatch.elapsed_secs();
}

pub fn update_ingame_viewmodel(
    typing_state:Res<session::TypingState>,
    mut ingame_viewmodel:ResMut<view_model::InGameViewModel>
){
    ingame_viewmodel.problem_index=typing_state.progress.problem_index;
}

pub fn update_problem_viewmodel(
    typing_state:Res<session::TypingState>,
    mut problem_viewmodel:ResMut<view_model::ProblemViewModel>
){
    problem_viewmodel.kana_index=typing_state.current_problem_session.kana_index;
    problem_viewmodel.input=typing_state.current_problem_session.inputbuf.clone();
}