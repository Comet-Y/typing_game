use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::time::Stopwatch;
use bevy::prelude::*;
#[derive(Clone, Debug, Default)]
struct TrieTree {
    trie: Vec<TrieNode>,
}

#[derive(Default, Clone, Debug)]
struct TrieNode {
    is_end_node: bool,
    is_n: bool,
    next_nodes: std::collections::HashMap<char, usize>,
}

#[derive(Resource)]
struct TypingState {
    tries: Vec<TrieTree>,
    problem_index: usize,
    problem_count: usize,
    problems: Vec<ProblemState>,
    kpm_data:KpmData,
}

struct ProblemState {
    odai: String,
    odai_kana: Vec<String>,
    product: Vec<usize>,
    inputbuf: String,
    kana_index: usize,
    node_index: usize,
}
struct KpmData{
    last_inputbuf_len:usize,
    typed_sum:usize,
}
#[derive(Resource, Clone)]
struct Sounds {
    correct: Handle<AudioSource>,
    miss: Handle<AudioSource>,
}

#[derive(Component)]
struct InputText;

#[derive(Component)]
struct TargetText;

#[derive(Component)]
struct ProblemIndexDisplay;

#[derive(Component)]
struct KanaSpan(usize);

#[derive(Resource, PartialEq)]
struct NOk(bool);

#[derive(Resource)]
struct TextParentEntity(Entity);
#[derive(Resource)]
struct TargetTextKanaEntity(Entity);
#[derive(Resource,Debug)]
struct Wait(Timer);

#[derive(Resource,Default)]
struct GameTimer{
    stopwatch:Stopwatch,
    prev:f32
}
#[derive(Resource)]
struct TextFontPreset(TextFont);
#[derive(Message)]
struct ProblemChanged;

#[derive(Message)]
struct DisplayKpm;

#[derive(States, Clone, Debug, Hash, Eq, PartialEq, Default)]
#[states(scoped_entities)]
enum GameState {
    #[default]
    Start,
    MainMenu,
    InGame,
    EndMenu,
}
impl Sounds{
    fn correct(&self,commands:&mut Commands){
         commands.spawn((
            AudioPlayer::new(self.correct.clone()),
            PlaybackSettings::DESPAWN
        ));
    }
    fn miss(&self, commands:&mut Commands){
        commands.spawn((
            AudioPlayer::new(self.miss.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}
impl TrieTree {
    fn new() -> Self {
        Self {
            trie: vec![TrieNode {
                is_end_node: false,
                ..Default::default()
            }],
        }
    }

    fn insert(&mut self, adds: &[&str]) {
        let mut index = self.trie.len();
        for &add in adds {
            let mut cur = 0;
            for c in add.chars() {
                if let Some(nextindex) = self.trie[cur].next_nodes.get(&c) {
                    cur = *nextindex;
                } else {
                    self.trie[cur].next_nodes.insert(c, index);
                    cur = index;
                    index += 1;
                    self.trie.push(TrieNode {
                        is_end_node: false,
                        ..Default::default()
                    });
                }
            }
            self.trie[cur].is_end_node = true;
        }
    }
}

impl std::fmt::Debug for TypingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "problem_index:{}\nproblem_count:{}",
            &self.problem_index, &self.problem_count,
        )
    }
}

impl std::fmt::Debug for ProblemState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "kana_index:{}\nnode_index:{}",
            &self.kana_index, &self.node_index,
        )
    }
}
impl TypingState {
    fn new(n: usize) -> Self {
        TypingState {
            tries: Vec::new(),
            problem_index: 0,
            problem_count: n,
            problems: Vec::new(),
            kpm_data:KpmData::new()
        }
    }
    fn advance(&mut self, c: char, nok: &mut NOk) -> std::result::Result<(), ()> {
        if let Some(next) = self.next_node(&c) {
            self.advance_node(*next);
            if self.is_end_node() {
                *nok = NOk(self.is_n());
                self.problems[self.problem_index].advance();
            }
            Ok(())
        } else if c == 'n' && *nok == NOk(true) {
            *nok = NOk(false);
            Ok(())
        } else {
            println!("{:?}", self);
            Err(())
        }
    }
    fn is_end_node(&self) -> bool {
        self.tries[self.now_trie()].trie[self.now_node()].is_end_node
    }

    fn is_n(&self) -> bool {
        self.tries[self.now_trie()].trie[self.now_node()].is_n
    }
    fn will_out_of_range(&self)->bool{
        self.problem_index>=self.problem_count
    }
    fn now_kana(&self) -> usize {
        if self.will_out_of_range(){
            return 0;
        }
        self.problems[self.problem_index].now_kana()
    }
    fn now_trie(&self) -> usize {
        if self.will_out_of_range(){
            return 0;
        }
        self.problems[self.problem_index].now_trie()
    }
    fn now_node(&self) -> usize {
        if self.will_out_of_range(){
            return 0;
        }
        self.problems[self.problem_index].now_node()
    }

    fn now_odai(&self) -> &str {
        if self.will_out_of_range(){
            return "";
        }
        self.problems[self.problem_index].now_odai()
    }
    fn now_odai_kana(&self) -> &Vec<String> {
        self.problems[self.problem_index].now_odai_kana()
    }
    fn now_inputbuf(&self) -> &str {
        if self.will_out_of_range(){
            return "";
        }
        self.problems[self.problem_index].now_inputbuf()
    }
    fn next_node(&self, c: &char) -> Option<&usize> {
        self.tries[self.now_trie()].trie[self.now_node()]
            .next_nodes
            .get(c)
    }
    fn goto_next_problem(&mut self) {

        self.kpm_data.last_inputbuf_len=self.now_inputbuf().len();
        self.kpm_data.accumulate();
        self.problems[self.problem_index].initialize();
        self.problem_index += 1;
    }

    fn initialize(&mut self) {
        self.kpm_data.initialize();
        self.problem_index = 0;
    }
    fn advance_node(&mut self, next: usize) {
        self.problems[self.problem_index].node_index = next;
    }
    fn one_problem_cleared(&self) -> bool {
        self.problems[self.problem_index].cleared()
    }
    fn cleared(&self) -> bool {
        self.problem_index == self.problem_count
    }
    fn add_inputbuf(&mut self, c: char) {
        self.problems[self.problem_index].inputbuf.push(c);
    }
    fn last_inputbuf_len(&self)->usize{
        self.kpm_data.last_inputbuf_len
    }
    fn typed_sum(&self)->usize{
        self.kpm_data.typed_sum
    }
}
impl ProblemState {
    fn new(odai: String, odai_kana: Vec<String>, product: Vec<usize>) -> Self {
        Self {
            odai,
            odai_kana,
            product,
            inputbuf: String::new(),
            kana_index: 0,
            node_index: 0,
        }
    }
    fn now_kana(&self) -> usize {
        self.kana_index
    }
    fn now_trie(&self) -> usize {
        self.product[self.kana_index]
    }
    fn now_node(&self) -> usize {
        self.node_index
    }
    fn now_odai(&self) -> &str {
        &self.odai
    }
    fn now_odai_kana(&self) -> &Vec<String> {
        &self.odai_kana
    }
    fn now_inputbuf(&self) -> &str {
        &self.inputbuf
    }
    fn initialize(&mut self) {
        (self.inputbuf, self.kana_index, self.node_index) = (String::new(), 0, 0);
    }
    fn advance(&mut self) {
        self.kana_index += 1;
        self.node_index = 0;
    }
    fn cleared(&self) -> bool {
        self.kana_index == self.product.len()
    }
}

impl KpmData{
    fn new()->Self{
        Self{
            last_inputbuf_len:0,
            typed_sum:0,
        }
    }
    fn initialize(&mut self){
        (self.last_inputbuf_len,self.typed_sum)=(0,0);
    }
    fn accumulate(&mut self){
        self.typed_sum+=self.last_inputbuf_len;
    }
}
impl GameTimer{
    fn duration(&self)->f32{
        self.stopwatch.elapsed_secs()-self.prev
    }
    fn setprev(&mut self){
        self.prev=self.stopwatch.elapsed_secs();
    }
    fn reset(&mut self){
        self.prev=0.0;
        self.stopwatch.reset()
    }
    fn time(&self)->f32{
        self.stopwatch.elapsed_secs()
    }
}
fn initialize() -> (Vec<TrieTree>, std::collections::HashMap<String, usize>) {
    let mut tries = Vec::new();
    let mut indexes = std::collections::HashMap::new();
    vec![
        vec!["a"],
        vec!["i", "yi"],
        vec!["u", "whu"],
        vec!["e"],
        vec!["o"], //0
        vec!["ka", "ca"],
        vec!["ki"],
        vec!["ku", "cu"],
        vec!["ke"],
        vec!["ko", "co"], //5
        vec!["sa"],
        vec!["si", "ci", "shi"],
        vec!["su"],
        vec!["se", "ce"],
        vec!["so"], //10
        vec!["ta"],
        vec!["ti", "chi"],
        vec!["tu", "tsu"],
        vec!["te"],
        vec!["to"], //15
        vec!["na"],
        vec!["ni"],
        vec!["nu"],
        vec!["ne"],
        vec!["no"], //20
        vec!["ha"],
        vec!["hi"],
        vec!["hu", "fu"],
        vec!["he"],
        vec!["ho"], //25
        vec!["ma"],
        vec!["mi"],
        vec!["mu"],
        vec!["me"],
        vec!["mo"], //30
        vec!["ya"],
        vec!["yu"],
        vec!["yo"], //35
        vec!["ra"],
        vec!["ri"],
        vec!["ru"],
        vec!["re"],
        vec!["ro"], //38
        vec!["wa"],
        vec!["wo"],
        vec!["xn", "nn"], //43
        vec!["ye", "ile", "ixe", "yile", "yixe"],
        vec!["wi", "whi", "uli", "uxi", "whuli", "whuxi"],
        vec!["we", "whe", "ule", "uxe", "whule", "whuxe"], //46
        vec!["kya", "kilya", "kixya"],
        vec!["kyu", "kilyu", "kixyu"],
        vec!["kyo", "kilyo", "kixyo"], //49
        vec!["sha", "sya", "silya", "sixya", "shilya", "shixya"],
        vec!["shu", "syu", "silyu", "sixyu", "shilyu", "shixyu"],
        vec!["sho", "syo", "silyo", "sixyo", "shilyo", "shixyo"], //52
        vec!["swa", "sula", "suxa"],
        vec!["swi", "suli", "suxi", "sulyi", "suxyi"],
        vec!["swu", "sulu", "suxu"],
        vec!["swe", "sule", "suxe"],
        vec!["swo", "sulo", "suxo"], //55
        vec!["tya", "cha", "cya", "tilya", "tixya", "chilya", "chixya"],
        vec![
            "tyi", "tili", "tixi", "chili", "chixi", "tilyi", "tixyi", "chilyi", "chixyi",
        ],
        vec!["tyu", "chu", "cyu", "tilyu", "tixyu", "chilyu", "chixyu"],
        vec!["tye", "che", "tile", "tixe", "chile", "chixe"],
        vec!["tyo", "cho", "cyo", "tilyo", "tixyo", "chilyo", "chixyo"], //60
        vec!["tha", "telya", "texya"],
        vec!["thi", "teli", "texi", "telyi", "texyi"],
        vec!["thu", "telu", "texu"],
        vec!["the", "tele", "texe"],
        vec!["tho", "telo", "texo"], //65
        vec!["nya", "nilya", "nixya"],
        vec!["nyi", "nili", "nixi", "nilyi", "nixyi"],
        vec!["nyu", "nilyu", "nilxyu"],
        vec!["nye", "nile", "nixe"],
        vec!["nyo", "nilyo", "nixyo"], //70
        vec!["hya", "hilya", "hixya"],
        vec!["hyi", "hili", "hixi", "hilyi", "hixyi"],
        vec!["hyu", "hilyu", "hixyu"],
        vec!["hye", "hile", "hixe"],
        vec!["hyo", "hilyo", "hixyo"], //75
        vec!["mya", "milya", "mixya"],
        vec!["myi", "mili", "mixi", "milyi", "mixyi"],
        vec!["myu", "milyu", "mixyu"],
        vec!["mye", "mile", "mixe"],
        vec!["myo", "milyo", "mixyo"], //80
        vec!["rya", "rilya", "rixya"],
        vec!["ryi", "rili", "rixi", "rilyi", "rixyi"],
        vec!["ryu", "rilyu", "rixyu"],
        vec!["rye", "rile", "rixe"],
        vec!["ryo", "rilyo", "rixyo"], //85
        vec!["la", "xa"],
        vec!["li", "xi", "lyi", "xyi"],
        vec!["lu", "xu", "lwhu", "xwhu"],
        vec!["le", "xe"],
        vec!["lo", "xo"], //90
        vec!["lya", "xya"],
        vec!["lyu", "xyu"],
        vec!["lyo", "xyo"], //95
        vec!["lwa", "xwa"], //98
        vec!["vu"],         //99
        vec!["ga"],
        vec!["gi"],
        vec!["gu"],
        vec!["ge"],
        vec!["go"], //100
        vec!["za"],
        vec!["ji", "zi"],
        vec!["zu"],
        vec!["ze"],
        vec!["zo"], //105
        vec!["da"],
        vec!["di"],
        vec!["du"],
        vec!["de"],
        vec!["do"], //110
        vec!["ba"],
        vec!["bi"],
        vec!["bu"],
        vec!["be"],
        vec!["bo"], //115
        vec!["va", "vula", "vuxa"],
        vec!["vi", "vuli", "vuxi", "vulyi", "vuxyi"],
        vec!["ve", "vule", "vuxe"],
        vec!["vo", "vulo", "vuxo"], //120
        vec!["gya", "gilya", "gixya"],
        vec!["gyi", "gili", "gixi", "gilyi", "gixyi"],
        vec!["gyu", "gilyu", "gixyu"],
        vec!["gye", "gile", "gixe"],
        vec!["gyo", "gilyo", "gixyo"], //124
        vec![
            "ja", "jya", "zya", "jilya", "jixya", "zya", "zilya", "zixya",
        ],
        vec![
            "jyi", "jili", "jixi", "jilyi", "jixyi", "zyi", "zili", "zixi", "zilyi", "zixyi",
        ],
        vec!["ju", "jyu", "jilyu", "jixyu", "zyu", "zilyu", "zixyu"],
        vec!["je", "jye", "jile", "jixe", "zye", "zile", "zixe"],
        vec!["jo", "jyo", "jilyo", "jixyo", "zyo", "zilyo", "zixyo"], //129
        vec!["dya", "dilya", "dixya"],
        vec!["dyi", "dili", "dixi", "dixyi", "dilyi"],
        vec!["dyu", "dilyu", "dixyu"],
        vec!["dye", "dile", "dixe"],
        vec!["dyo", "dilyo", "dixyo"], //134
        vec!["dha", "delya", "dexya"],
        vec!["dhi", "deli", "dexi", "delyi", "dexyi"],
        vec!["dhu", "delyu", "dexyu"],
        vec!["dhe", "dele", "dexe"],
        vec!["dho", "delo", "dexo"], //139
        vec!["n", "xn"],             //144
    ]
    .iter()
    .for_each(|s| {
        let mut t = TrieTree::new();
        t.insert(&s);
        tries.push(t);
    });
    tries[144].trie[1].is_n = true;
    [
        "あ", "い", "う", "え", "お", "か", "き", "く", "け", "こ", "さ", "し", "す", "せ", "そ",
        "た", "ち", "つ", "て", "と", "な", "に", "ぬ", "ね", "の", "は", "ひ", "ふ", "へ", "ほ",
        "ま", "み", "む", "め", "も", "や", "ゆ", "よ", "ら", "り", "る", "れ", "ろ", "わ", "を",
        "んa", "いぇ", "うぃ", "うぇ", "きゃ", "きゅ", "きょ", "しゃ", "しゅ", "しょ", "すぁ",
        "すぃ", "すぅ", "すぇ", "すぉ", "ちゃ", "ちぃ", "ちゅ", "ちぇ", "ちょ", "てゃ", "てぃ",
        "てゅ", "てぇ", "てょ", "にゃ", "にぃ", "にゅ", "にぇ", "にょ", "ひゃ", "ひぃ", "ひゅ",
        "ひぇ", "ひょ", "みゃ", "みぃ", "みゅ", "みぇ", "みょ", "りゃ", "りぃ", "りゅ", "りぇ",
        "りょ", "ぁ", "ぃ", "ぅ", "ぇ", "ぉ", "ゃ", "ゅ", "ょ", "ゎ", "ゔ", "が", "ぎ", "ぐ", "げ",
        "ご", "ざ", "じ", "ず", "ぜ", "ぞ", "だ", "ぢ", "づ", "で", "ど", "ば", "び", "ぶ", "べ",
        "ぼ", "ゔぁ", "ゔぃ", "ゔぇ", "ゔぉ", "ぎゃ", "ぎぃ", "ぎゅ", "ぎぇ", "ぎょ", "じゃ",
        "じぃ", "じゅ", "じぇ", "じょ", "ぢゃ", "ぢぃ", "ぢゅ", "ぢぇ", "ぢょ", "でゃ", "でぃ",
        "でゅ", "でぇ", "でょ", "んb",
    ]
    .iter()
    .enumerate()
    .for_each(|(i, &s)| {
        indexes.insert(s.to_string(), i);
    });

    (tries, indexes)
}

fn build_product(
    input: &str,
    indexes: &std::collections::HashMap<String, usize>,
) -> (Vec<usize>, Vec<String>) {
    let mut product = Vec::new();
    let mut odai_kana = Vec::new();
    let mut skip = false;
    let input = format!("{}{}", input.trim(), "x");
    input
        .chars()
        .zip(input.chars().skip(1))
        .for_each(|(one, two)| {
            if skip {
                skip = false;
                return;
            }
            if let Some(index) = indexes.get(&format! {"{}{}",one,two}) {
                skip = true;
                product.push(*index);
                odai_kana.push(format!("{}{}", one, two));
            } else {
                if let Some(index) = indexes.get(&one.to_string()) {
                    product.push(*index);
                    odai_kana.push(one.to_string());
                } else if one == 'ん' {
                    if "あいうえおなにぬねのやゆよ".contains(two) {
                        product.push(*indexes.get("んa").unwrap());
                    } else {
                        product.push(*indexes.get("んb").unwrap());
                    }
                    odai_kana.push("ん".to_string());
                } else {
                    println!("{}{}", one, two);
                    panic!("ERROR:couldn't make trie tree");
                }
            }
        });
    (product, odai_kana)
}
fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    let mut typingstate = TypingState::new(n);
    let (tries, indexes) = initialize();
    typingstate.tries = tries;
    for _ in 0..n {
        let mut odai = String::new();
        let mut kana = String::new();
        std::io::stdin().read_line(&mut odai).unwrap();
        std::io::stdin().read_line(&mut kana).unwrap();
        let (product, odai_kana) = build_product(&kana, &indexes);
        typingstate.problems.push(ProblemState::new(
            odai.trim().to_string(),
            odai_kana,
            product,
        ));
    }
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(NOk(false))
        .insert_resource(typingstate)
        .add_systems(OnEnter(GameState::Start), setup)
        .add_systems(OnEnter(GameState::MainMenu), start_mainmenu)
        .add_systems(
            FixedUpdate,
            start_game.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(OnEnter(GameState::InGame), game_setup)
        .add_systems(
            FixedUpdate,
            (change_odai,change_odai_kana,change_problem_index_display, change_ui, handle_key,erase_ui,display_kpm)
                .chain()
                .run_if(in_state(GameState::InGame).and_then(can_continue)
                ).after(update_wait)
        )
        .add_systems(OnEnter(GameState::EndMenu), start_endmenu)
        .add_systems(
            FixedUpdate,
            return_to_mainmenu.run_if(in_state(GameState::EndMenu)),
        )
        .add_systems(FixedUpdate,update_wait.run_if(in_state(GameState::InGame)))
        .add_message::<ProblemChanged>()
        .add_message::<DisplayKpm>()
        .run();
}
fn setup(
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    
    commands.insert_resource(Wait(Timer::from_seconds(1.0,TimerMode::Once)));
    commands.insert_resource(GameTimer{
        stopwatch:Stopwatch::new(),
        prev:0.0
    });
    let japanese_font=asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");
    commands.insert_resource(TextFontPreset(TextFont{
            font: japanese_font.clone().into(),
                font_size: FontSize::Px(50.0),
                ..default()
    }));
    commands.spawn(Camera2d);
    let parent = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        })
        .id();
    commands.insert_resource(TextParentEntity(parent));
    next_state.set(GameState::MainMenu);
}

fn start_mainmenu(mut commands: Commands, parent: Res<TextParentEntity>) {
    let prompt_start = commands
        .spawn((
            Text::new("Press Space key to start"),
            DespawnOnExit(GameState::MainMenu),
        ))
        .id();
    commands.entity(parent.0).add_child(prompt_start);
}
fn start_game(
    mut next_state: ResMut<NextState<GameState>>,
    mut msg_kbd: MessageReader<KeyboardInput>,
) {
    for msg in msg_kbd.read() {
        if msg.state == ButtonState::Released {
            continue;
        }
        match msg.logical_key {
            Key::Space => {
                next_state.set(GameState::InGame);
            }
            _ => {
                continue;
            }
        }
    }
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parent: Res<TextParentEntity>,
    mut game_timer:ResMut<GameTimer>,
    mut wait:ResMut<Wait>,
    text_font_preset:Res<TextFontPreset>,
) {
    wait.0.reset();
    wait.0.unpause();
    game_timer.reset();
    commands.spawn((
        Text::new(""),
        DespawnOnExit(GameState::InGame),
        text_font_preset.0.clone(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ProblemIndexDisplay,
    ));
    let target_text = commands
        .spawn((
            Text::new(""),
            DespawnOnExit(GameState::InGame),
            TargetText,
            text_font_preset.0.clone(),
        ))
        .id();
    commands.entity(parent.0).add_child(target_text);
    let target_kana = commands
        .spawn((Text::new(""), DespawnOnExit(GameState::InGame)))
        .id();
    commands.entity(parent.0).add_child(target_kana);
    commands.insert_resource(TargetTextKanaEntity(target_kana));
    let input = commands
        .spawn((
            Text::new(""),
            InputText,
            DespawnOnExit(GameState::InGame),
            text_font_preset.0.clone(),
        ))
        .id();
    commands.entity(parent.0).add_child(input);
    let correct = asset_server.load("sounds/correct.mp3");
    let miss = asset_server.load("sounds/miss.mp3");
    commands.insert_resource(Sounds { correct, miss });
}

fn update_wait(
    time:Res<Time>,
    mut wait:ResMut<Wait>,
    mut game_timer:ResMut<GameTimer>
){
    wait.0.tick(time.delta());
    game_timer.stopwatch.tick(time.delta());
}
fn change_ui(
    typingstate: Res<TypingState>,
    mut target_kana: Query<
        (&mut TextColor, &KanaSpan),
        (Without<InputText>, Without<ProblemIndexDisplay>),
    >,
    mut input: Query<
        &mut Text,
        (
            With<InputText>,
            Without<KanaSpan>,
            Without<ProblemIndexDisplay>,
        ),
    >,
) {
    target_kana.iter_mut().for_each(|(mut color, span)| {
        if span.0 < typingstate.now_kana() {
            color.0 = Color::srgb(0.5, 0.5, 0.5);
        }
    });
    if let Ok(mut text) = input.single_mut() {
        *text = Text::new(typingstate.now_inputbuf());
    }
}

fn display_kpm(
    mut msgs:MessageReader<DisplayKpm>,
    mut game_timer:ResMut<GameTimer>,
    typingstate:Res<TypingState>,
    mut target:Query<&mut Text,With<TargetText>>,
    mut wait:ResMut<Wait>,
){
    for _ in msgs.read(){
    game_timer.stopwatch.pause();
    let kpm=60.0*typingstate.last_inputbuf_len() as f32/(game_timer.duration());
    if let Ok(mut text)=target.single_mut(){
        *text=Text::new(format!("{}  kpm",kpm.to_string()));
    }
    game_timer.setprev();
    wait.0.reset();
    wait.0.unpause();
}
}

fn erase_ui(
    mut msgs:MessageReader<DisplayKpm>,
    mut commands:Commands,
     target_kana:ResMut<TargetTextKanaEntity>,
     mut input:Query<&mut Text,With<InputText>>,
){  
    for _ in msgs.read(){
    commands.entity(target_kana.0).despawn_children();
    commands.entity(target_kana.0).insert(Text::new(""));
    if let Ok(mut text)=input.single_mut(){
        *text=Text::new("");
    }
}
}
fn can_continue(
    wait:Res<Wait>,
)->bool{
    wait.0.is_finished()
}
fn change_odai(
    mut msgs:MessageReader<ProblemChanged>,
    typingstate:Res<TypingState>,
    mut target:Query<&mut Text,(With<TargetText>,Without<KanaSpan>,Without<InputText>)>
){
    for _ in msgs.read(){
        if let Ok(mut text)=target.single_mut(){
            *text=Text::new(typingstate.now_odai());
        }
    }
}

fn change_problem_index_display(
    mut msgs:MessageReader<ProblemChanged>,
    typingstate:Res<TypingState>,
    mut problem_index_display:Query<&mut Text,(With<ProblemIndexDisplay>,Without<KanaSpan>,Without<InputText>)>
){
    for _ in msgs.read(){
        if let Ok(mut text)=problem_index_display.single_mut(){
            *text=Text::new(format!("{}/{}",typingstate.problem_index+1,typingstate.problem_count));
        }
    }
}

fn change_odai_kana(
    mut msgs: MessageReader<ProblemChanged>,
    mut commands: Commands,
    target_kana: ResMut<TargetTextKanaEntity>,
    typingstate: Res<TypingState>,
    text_font_preset:Res<TextFontPreset>
) {
    for _ in msgs.read() {
        for (i, s) in typingstate.now_odai_kana().iter().enumerate() {
            let child = commands
                .spawn((
                    TextSpan::new(s.clone()),
                    DespawnOnExit(GameState::InGame),
                    text_font_preset.0.clone(),
                    TextColor(Color::WHITE),
                    KanaSpan(i),
                ))
                .id();
            commands.entity(target_kana.0).add_child(child);
        }
    }
}

fn handle_key(
    sounds: Res<Sounds>,
    mut commands: Commands,
    mut msg_kbd: MessageReader<KeyboardInput>,
    mut typingstate: ResMut<TypingState>,
    mut nok: ResMut<NOk>,
    mut msg_problem_changed: MessageWriter<ProblemChanged>,
    mut msg_display_wpm:MessageWriter<DisplayKpm>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_timer:ResMut<GameTimer>,
    wait:ResMut<Wait>
) {
    if wait.0.just_finished(){
        msg_problem_changed.write(ProblemChanged);
    }
    
    if typingstate.cleared() {
        next_state.set(GameState::EndMenu);
        return;
    }
    game_timer.stopwatch.unpause();
    for msg in msg_kbd.read() {
        if msg.state == ButtonState::Released {
            continue;
        }
        if msg.repeat {
            continue;
        }
        if let Key::Character(input) = &msg.logical_key {
            for c in input.chars() {
                match typingstate.advance(c, &mut nok) {
                    Ok(()) => {
                        sounds.correct(&mut commands);
                        typingstate.add_inputbuf(c);
                    }
                    Err(()) => {
                        sounds.miss(&mut commands);
                    }
                }
                if typingstate.one_problem_cleared() {
                    typingstate.goto_next_problem();
                    msg_display_wpm.write(DisplayKpm);

                }
            }
        }
    }
}

fn start_endmenu(
    mut commands: Commands,
    parent: Res<TextParentEntity>,
    typingstate:Res<TypingState>,
    game_timer:Res<GameTimer>,
    ) {

    let kpm=commands.spawn((
        Text::new(format!("{}  kpm",(60.0*typingstate.typed_sum() as f32/game_timer.time()).to_string())),
        DespawnOnExit(GameState::EndMenu),
    )
    ).id();

    let clear = commands
        .spawn((Text::new("CLEAR!"), DespawnOnExit(GameState::EndMenu)))
        .id();
    commands.entity(parent.0).add_child(kpm);
    commands.entity(parent.0).add_child(clear);
}
fn return_to_mainmenu(
    mut next_state: ResMut<NextState<GameState>>,
    mut msg_kbd: MessageReader<KeyboardInput>,
    mut exit: MessageWriter<AppExit>,
    mut typingstate: ResMut<TypingState>,
) {
    for msg in msg_kbd.read() {
        match msg.logical_key {
            Key::Space => {
                typingstate.initialize();
                next_state.set(GameState::MainMenu);
            }
            Key::Enter => {
                exit.write(AppExit::Success);
            }
            _ => {
                continue;
            }
        }
    }
}
