use bevy::prelude::*;
#[derive(Resource)]
pub struct InGameViewModel{
    pub problem_index:usize,
    pub problem_count:usize,
}
#[derive(Resource)]
pub struct ProblemViewModel{
    pub odai:String,
    pub odai_kana:Vec<String>,
    pub kana_index:usize,
    pub input:String,
}
#[derive(Resource)]
pub struct KpmViewModel{
  pub kpm:String,
}
#[derive(Resource)]
pub struct EndMenuViewModel{
    pub kpm:String,
}