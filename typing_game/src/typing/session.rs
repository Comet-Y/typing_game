use bevy::prelude::*;
use crate::typing::trie;
use crate::typing::romaji;
#[derive(Debug)]
pub struct KanaUnit{
    pub kana_chunk:String,
    trie_id:romaji::TrieId,
}
impl KanaUnit{
    pub fn new(kana_chunk:String,trie_id:romaji::TrieId)->Self{
        Self{kana_chunk,trie_id}
    }
}

#[derive(Resource,Debug)]
pub struct TypingState {
    pub progress:Progress,
    pub problems: Vec<Problem>,
    pub current_problem_session:ProblemSession,
    pub kpm_data:KpmData,
}
pub struct Progress{
    pub problem_index:usize,
    pub problem_count:usize,
}
impl Progress{
    fn new()->Self{
        Self { problem_index:0, problem_count:0 }
    }
    pub fn return_to_start(&mut self){
        self.problem_index=0;
    }
    pub fn is_clear(&self)->bool{
        self.problem_index==self.problem_count
    }
}
#[derive(Debug)]
pub struct Problem {
    pub odai: String,
    pub kana_units: Vec<KanaUnit>,
}

pub struct ProblemSession{
    pub inputbuf:String,
    pub kana_index:usize,
    node_index:usize,
    can_type_n:bool,
}
impl ProblemSession{
    fn new()->Self{
        Self { inputbuf:String::new(), kana_index:0, node_index:0,can_type_n:false }
    }
    fn enable_n(&mut self){
        self.can_type_n=true;
    }
    fn disable_n(&mut self){
        self.can_type_n=false;
    }
    fn advance(&mut self){
        self.kana_index+=1;
        self.node_index=0;
    }
    fn initialize(&mut self){
        (self.inputbuf,self.kana_index,self.node_index,self.can_type_n)=(String::new(),0,0,false);
    }
}
#[derive(Debug)]
pub struct KpmData{
    pub last_inputbuf_len:usize,
    pub typed_sum:usize,
}

impl std::fmt::Debug for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "problem_index:{}\nproblem_count:{}",
            &self.problem_index, &self.problem_count,
        )
    }
}

impl std::fmt::Debug for ProblemSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "kana_index:{}\nnode_index:{}",
            &self.kana_index, &self.node_index,
        )
    }
}
impl TypingState {
    pub fn new() -> Self {
        TypingState {
            progress:Progress::new(),
            problems: Vec::new(),
            current_problem_session:ProblemSession::new(),
            kpm_data:KpmData::new()

        }
    }
    pub fn advance(&mut self,dict:&romaji::RomajiDictionary, c: char) -> std::result::Result<(), ()> {
        if let Some(next) = self.next_node(c,dict) {
            self.current_problem_session.inputbuf.push(c);
            self.advance_node(*next);
            if self.is_end_node(dict) {
                self.try_enable_n(dict);
                self.current_problem_session.advance();
            }
            Ok(())
        } else if c == 'n' && self.current_problem_session.can_type_n {
            self.current_problem_session.inputbuf.push(c);
            self.current_problem_session.disable_n();
            Ok(())
        } else {
            Err(())
        }
    }
    fn is_end_node(&self,dict:&romaji::RomajiDictionary) -> bool {
        self.runtime_node(dict).is_end_node
    }
    
    pub fn is_clear(&self)->bool{
        self.get_current_problem().kana_units.len()==self.current_problem_session.kana_index
    }

    fn is_n(&self,dict:&romaji::RomajiDictionary) -> bool {
        self.runtime_node(dict).is_n
    }
    fn next_node<'a>(&self, c: char,dict:&'a romaji::RomajiDictionary) -> Option<&'a usize> {
        self.runtime_node(dict)
            .next_nodes
            .get(&c)
    }
    pub fn goto_next_problem(&mut self) {
        self.kpm_data.last_inputbuf_len=self.current_problem_session.inputbuf.len();
        self.kpm_data.accumulate();
        self.current_problem_session.initialize();
        self.progress.problem_index += 1;
    }

    pub fn return_to_start(&mut self) {
        self.kpm_data.initialize();
        self.progress.return_to_start();
    }
    fn advance_node(&mut self, next: usize) {
        self.current_problem_session.node_index = next;
    }

    pub fn get_current_problem(&self)->&Problem{
        &self.problems[self.progress.problem_index]
    }
    fn runtime_trie<'a>(&self,dict:&'a romaji::RomajiDictionary)->&'a trie::TrieTree{
        let problem=self.get_current_problem();
        let problem_session=&self.current_problem_session;
        dict.get_trie(problem.kana_units[problem_session.kana_index].trie_id)
    }
    fn runtime_node<'a>(&self,dict:&'a romaji::RomajiDictionary)->&'a trie::TrieNode{
        let problem_session=&self.current_problem_session;
        let trie=self.runtime_trie(dict);
        &trie.trie[problem_session.node_index]
    }
    fn try_enable_n(&mut self,dict:&romaji::RomajiDictionary){
        if self.is_n(dict){
            self.current_problem_session.enable_n();
        }
    }
   pub  fn add_odai(
        &mut self,
        odai:&str,
        odai_kana:&str,
        dict:&Res<romaji::RomajiDictionary>
    ){
        let kana_units=dict.build_kana_units(odai_kana);
        let problem=Problem::new(odai.to_string(),kana_units);
        self.problems.push(problem);
        self.progress.problem_count+=1;
    }
}
impl Problem {
    fn new(odai: String, kana_units: Vec<KanaUnit>) -> Self {
        Self {
            odai,
            kana_units,
        }
    }
}

impl KpmData{
    fn new()->Self{
        Self{
            last_inputbuf_len:0,
            typed_sum:0,
        }
    }
    pub fn initialize(&mut self){
        (self.last_inputbuf_len,self.typed_sum)=(0,0);
    }
    fn accumulate(&mut self){
        self.typed_sum+=self.last_inputbuf_len;
    }
}