use bevy::prelude::*;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key,KeyboardInput};
#[derive(Clone,Debug,Default)]
struct TrieTree{
    trie:Vec<TrieNode>,
}

#[derive(Default,Clone,Debug)]
struct TrieNode{
    is_end_node:bool,
    is_n:bool,
    next_nodes: std::collections::HashMap<char,usize>
}

#[derive(Resource)]
struct TypingState{
    inputbuf:String,
    tries:Vec<TrieTree>,
    odai_kana:Vec<Vec<String>>,
    product:Vec<Vec<usize>>,
    problem_index:usize,
    kana_index: usize,
    node_index: usize,
    problem_count: usize,
}

#[derive(Resource,Clone)]
struct SoundAssets{
    correct:Handle<AudioSource>,
    miss:Handle<AudioSource>,
}

#[derive(Resource,Clone)]
struct JapaneseFont(Handle<Font>);


#[derive(Component)]
struct InputText;

#[derive(Component)]
struct KanaSpan(usize);

#[derive(Resource,PartialEq)]
struct NOk(bool);

#[derive(Resource)]
struct TextParentEntity(Entity);
#[derive(Resource)]
struct TargetTextEntity(Entity);

#[derive(Message)]
struct ProblemChanged;

#[derive(States,Clone,Debug,Hash,Eq,PartialEq,Default)]
#[states(scoped_entities)]
enum GameState{
    #[default]
    Start,

    MainMenu,
    InGame,
    EndMenu,
}
impl TrieTree{
    fn new()->Self{
        Self{
            trie:vec![TrieNode{is_end_node:false,..Default::default()}],
        }
    }

    fn insert(&mut self,adds:&[&str]){
        let mut index=self.trie.len();
        for &add in adds{
            let mut cur=0;
            for c in add.chars(){
                if let Some(nextindex)=self.trie[cur].next_nodes.get(&c){
                    cur=*nextindex;
                }else{
                    self.trie[cur].next_nodes.insert(c,index);
                    cur=index;
                    index+=1;
                    self.trie.push(TrieNode{is_end_node:false,..Default::default()});
                }
            }
            self.trie[cur].is_end_node=true;
        }
    }
}

impl std::fmt::Debug for TypingState{
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(
            f,
            "problem_index:{}\nkana_index:{}\nnode_index:{}\nproblem_count:{}",
            &self.problem_index,
            &self.kana_index,
            &self.node_index,
            &self.problem_count,
        )
    }
}

impl TypingState{
    fn new(n:usize)->Self{
        TypingState{
            inputbuf:String::new(),
            tries:Vec::new(),
            odai_kana:Vec::new(),
            product:Vec::new(),
            problem_index:0,
            kana_index:0,
            node_index:0,
            problem_count:n,
        }
    }
    fn advance(&mut self,c:char,nok:&mut NOk)->std::result::Result<(),()>{
        if let Some(next)=self.next_node(&c){
            self.node_index=*next;
            if self.is_end_node(){
                *nok=NOk(self.is_n());
                self.node_index=0;
                self.kana_index+=1;
            }
            Ok(())
        }else if c=='n'&&*nok==NOk(true){
            *nok=NOk(false);
            Ok(())
        }
        else{
            println!("{:?}",self);
            Err(())
        }
    }
    fn is_end_node(&self)->bool{
        self.tries[self.product[self.problem_index][self.kana_index]].trie[self.node_index].is_end_node
    }

    fn is_n(&self)->bool{
        self.tries[self.product[self.problem_index][self.kana_index]].trie[self.node_index].is_n
    }

    fn next_node(&self,c:&char)->Option<&usize>{
        self.tries[self.product[self.problem_index][self.kana_index]].trie[self.node_index].next_nodes.get(c)
    }
    fn goto_next_problem(&mut self){
        self.problem_index+=1;
        self.kana_index=0;
        self.node_index=0;
    }
    fn initialize(&mut self){
        self.inputbuf=String::new();
        self.problem_index=0;
        self.kana_index=0;
        self.node_index=0;
    }
}

fn initialize()->(Vec<TrieTree>,std::collections::HashMap<String,usize>){
    let mut tries=Vec::new();
    let mut indexes=std::collections::HashMap::new();
    vec![vec!["a"],vec!["i","yi"],vec!["u","whu"],vec!["e"],vec!["o"],  //0
         vec!["ka","ca"],vec!["ki"],vec!["ku","cu"],vec!["ke"],vec!["ko","co"],//5
         vec!["sa"],vec!["si","ci","shi"],vec!["su"],vec!["se","ce"],vec!["so"],//10
         vec!["ta"],vec!["ti","chi"],vec!["tu","tsu"],vec!["te"],vec!["to"],//15
         vec!["na"],vec!["ni"],vec!["nu"],vec!["ne"],vec!["no"],//20
         vec!["ha"],vec!["hi"],vec!["hu","fu"],vec!["he"],vec!["ho"],//25
         vec!["ma"],vec!["mi"],vec!["mu"],vec!["me"],vec!["mo"],//30
         vec!["ya"],vec!["yu"],vec!["yo"],//35
         vec!["ra"],vec!["ri"],vec!["ru"],vec!["re"],vec!["ro"],//38
         vec!["wa"],vec!["wo"],vec!["xn","nn"],//43
         vec!["ye","ile","ixe","yile","yixe"],vec!["wi","whi","uli","uxi","whuli","whuxi"],vec!["we","whe","ule","uxe","whule","whuxe"],//46
         vec!["kya","kilya","kixya"],vec!["kyu","kilyu","kixyu"],vec!["kyo","kilyo","kixyo"],//49
         vec!["sha","sya","silya","sixya","shilya","shixya"],vec!["shu","syu","silyu","sixyu","shilyu","shixyu"],vec!["sho","syo","silyo","sixyo","shilyo","shixyo"],//52
         vec!["swa","sula","suxa"],vec!["swi","suli","suxi","sulyi","suxyi"],vec!["swu","sulu","suxu"],vec!["swe","sule","suxe"],vec!["swo","sulo","suxo"],//55
         vec!["tya","cha","cya","tilya","tixya","chilya","chixya"],vec!["tyi","tili","tixi","chili","chixi","tilyi","tixyi","chilyi","chixyi"],vec!["tyu","chu","cyu","tilyu","tixyu","chilyu","chixyu"],vec!["tye","che","tile","tixe","chile","chixe"],vec!["tyo","cho","cyo","tilyo","tixyo","chilyo","chixyo"],//60
         vec!["tha","telya","texya"],vec!["thi","teli","texi","telyi","texyi"],vec!["thu","telu","texu"],vec!["the","tele","texe"],vec!["tho","telo","texo"],//65
         vec!["nya","nilya","nixya"],vec!["nyi","nili","nixi","nilyi","nixyi"],vec!["nyu","nilyu","nilxyu"],vec!["nye","nile","nixe"],vec!["nyo","nilyo","nixyo"],//70
         vec!["hya","hilya","hixya"],vec!["hyi","hili","hixi","hilyi","hixyi"],vec!["hyu","hilyu","hixyu"],vec!["hye","hile","hixe"],vec!["hyo","hilyo","hixyo"],//75
         vec!["mya","milya","mixya"],vec!["myi","mili","mixi","milyi","mixyi"],vec!["myu","milyu","mixyu"],vec!["mye","mile","mixe"],vec!["myo","milyo","mixyo"],//80
         vec!["rya","rilya","rixya"],vec!["ryi","rili","rixi","rilyi","rixyi"],vec!["ryu","rilyu","rixyu"],vec!["rye","rile","rixe"],vec!["ryo","rilyo","rixyo"],//85
         vec!["la","xa"],vec!["li","xi","lyi","xyi"],vec!["lu","xu","lwhu","xwhu"],vec!["le","xe"],vec!["lo","xo"],//90
         vec!["lya","xya"],vec!["lyu","xyu"],vec!["lyo","xyo"],//95
         vec!["lwa","xwa"],//98
         vec!["vu"],//99
         vec!["ga"],vec!["gi"],vec!["gu"],vec!["ge"],vec!["go"],//100
         vec!["za"],vec!["ji","zi"],vec!["zu"],vec!["ze"],vec!["zo"],//105
         vec!["da"],vec!["di"],vec!["du"],vec!["de"],vec!["do"],//110
         vec!["ba"],vec!["bi"],vec!["bu"],vec!["be"],vec!["bo"],//115
         vec!["va","vula","vuxa"],vec!["vi","vuli","vuxi","vulyi","vuxyi"],vec!["ve","vule","vuxe"],vec!["vo","vulo","vuxo"],//120
         vec!["gya","gilya","gixya"],vec!["gyi","gili","gixi","gilyi","gixyi"],vec!["gyu","gilyu","gixyu"],vec!["gye","gile","gixe"],vec!["gyo","gilyo","gixyo"],//124
         vec!["ja","jya","zya","jilya","jixya","zya","zilya","zixya"],vec!["jyi","jili","jixi","jilyi","jixyi","zyi","zili","zixi","zilyi","zixyi"],vec!["ju","jyu","jilyu","jixyu","zyu","zilyu","zixyu"],vec!["je","jye","jile","jixe","zye","zile","zixe"],vec!["jo","jyo","jilyo","jixyo","zyo","zilyo","zixyo"],//129
         vec!["dya","dilya","dixya"],vec!["dyi","dili","dixi","dixyi","dilyi"],vec!["dyu","dilyu","dixyu"],vec!["dye","dile","dixe"],vec!["dyo","dilyo","dixyo"],//134
         vec!["dha","delya","dexya"],vec!["dhi","deli","dexi","delyi","dexyi"],vec!["dhu","delyu","dexyu"],vec!["dhe","dele","dexe"],vec!["dho","delo","dexo"],//139
         vec!["n","xn"]//144
     ]
    .iter()
    .for_each(|s|{
        let mut t=TrieTree::new();
        t.insert(&s);
        tries.push(t);
    });
    tries[144].trie[1].is_n=true;
    ["あ","い","う","え","お",
     "か","き","く","け","こ",
     "さ","し","す","せ","そ",
     "た","ち","つ","て","と",
     "な","に","ぬ","ね","の",
     "は","ひ","ふ","へ","ほ",
     "ま","み","む","め","も",
     "や","ゆ","よ",
     "ら","り","る","れ","ろ",
     "わ","を","んa",
     "いぇ","うぃ","うぇ",
     "きゃ","きゅ","きょ",
     "しゃ","しゅ","しょ",
     "すぁ","すぃ","すぅ","すぇ","すぉ",
     "ちゃ","ちぃ","ちゅ","ちぇ","ちょ",
     "てゃ","てぃ","てゅ","てぇ","てょ",
     "にゃ","にぃ","にゅ","にぇ","にょ",
     "ひゃ","ひぃ","ひゅ","ひぇ","ひょ",
     "みゃ","みぃ","みゅ","みぇ","みょ",
     "りゃ","りぃ","りゅ","りぇ","りょ",
     "ぁ","ぃ","ぅ","ぇ","ぉ",
     "ゃ","ゅ","ょ",
     "ゎ",
     "ゔ",
     "が","ぎ","ぐ","げ","ご",
     "ざ","じ","ず","ぜ","ぞ",
     "だ","ぢ","づ","で","ど",
     "ば","び","ぶ","べ","ぼ",
     "ゔぁ","ゔぃ","ゔぇ","ゔぉ",
     "ぎゃ","ぎぃ","ぎゅ","ぎぇ","ぎょ",
     "じゃ","じぃ","じゅ","じぇ","じょ",
     "ぢゃ","ぢぃ","ぢゅ","ぢぇ","ぢょ",
     "でゃ","でぃ","でゅ","でぇ","でょ",
     "んb"
     ].iter().enumerate()
    .for_each(|(i,&s)|{
        indexes.insert(s.to_string(),i);
    });

    (tries,indexes)
}

fn build_product(input:&str,indexes:&std::collections::HashMap<String,usize>)->(Vec<usize>,Vec<String>){

    let mut product=Vec::new();
    let mut odai_kana=Vec::new();
    let mut skip=false;
    let input=format!("{}{}",input.trim(),"x");
    input.chars().zip(input.chars().skip(1)).for_each(|(one,two)|{
        if skip{
            skip=false;
            return;
        }
        if let Some(index)=indexes.get(&format!{"{}{}",one,two}){
            skip=true;
            product.push(*index);
            odai_kana.push(format!("{}{}",one,two));
        }else{
            if let Some(index)=indexes.get(&one.to_string()){
                product.push(*index);
                odai_kana.push(one.to_string());
            }
            else if one=='ん'{
                if "あいうえおなにぬねのやゆよ".contains(two){
                    product.push(*indexes.get("んa").unwrap());
                }else{
                    product.push(*indexes.get("んb").unwrap());
                }
                odai_kana.push("ん".to_string());
            }else{
                println!("{}{}",one,two);
                panic!("ERROR:couldn't make trie tree");
            }
        }
    });
    (product,odai_kana)
}
fn main(){
    let mut n=String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n=n.trim().parse::<usize>().unwrap();
    let mut typingstate=TypingState::new(n);
    let (tries,indexes)=initialize();
    typingstate.tries=tries;
    for _ in 0..n{
        let mut input=String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let (product,odai_kana)=build_product(&input,&indexes);
        typingstate.product.push(product);
        typingstate.odai_kana.push(odai_kana);
    }
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(NOk(false))
        .insert_resource(typingstate)
        .add_systems(OnEnter(GameState::Start),setup)
        .add_systems(OnEnter(GameState::MainMenu),start_mainmenu)
        .add_systems(FixedUpdate,start_game.run_if(in_state(GameState::MainMenu)))
        .add_systems(OnEnter(GameState::InGame) ,game_setup)
        .add_systems(FixedUpdate,(handle_key,change_ui,change_problem).chain().run_if(in_state(GameState::InGame)))
        .add_systems(OnEnter(GameState::EndMenu),start_endmenu)
        .add_systems(FixedUpdate,return_to_mainmenu.run_if(in_state(GameState::EndMenu)))
        .add_message::<ProblemChanged>()
        .run();
}
fn setup(
    mut next_state:ResMut<NextState<GameState>>,
    mut commands:Commands,
    asset_server:Res<AssetServer>,
){
    commands.insert_resource(JapaneseFont(asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf")));
    commands.spawn(Camera2d);
    let parent=commands.spawn(Node{
        width:Val::Percent(100.0),
        height:Val::Percent(100.0),
        justify_content:JustifyContent::Center,
        align_items:AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(20.0),
        ..default()
    }).id();
    commands.insert_resource(TextParentEntity(parent));
    next_state.set(GameState::MainMenu);
}

fn start_mainmenu(
    mut commands:Commands,
    parent:Res<TextParentEntity>
){  
    let prompt_start=commands.spawn((
        Text::new("Press Space key to start"),
        DespawnOnExit(GameState::MainMenu),
    )
    ).id();
    commands.entity(parent.0).add_child(prompt_start);
}
fn start_game(
    mut next_state:ResMut<NextState<GameState>>,
    mut msg_kbd:MessageReader<KeyboardInput>
){
    for msg in msg_kbd.read(){
        if msg.state==ButtonState::Released{continue;}
        match msg.logical_key{
            Key::Space=>{
                next_state.set(GameState::InGame);
            },
            _=>{continue;}
        }
    }
}

fn game_setup(
    mut commands:Commands,
    asset_server:Res<AssetServer>,
    japanese_font:Res<JapaneseFont>,
    parent:Res<TextParentEntity>,
    mut msg_problem_changed:MessageWriter<ProblemChanged>
){  


    let target=commands.spawn(
        (
            Text::new(""),
            DespawnOnExit(GameState::InGame))).id();
    commands.entity(parent.0).add_child(target);
    commands.insert_resource(TargetTextEntity(target));
    msg_problem_changed.write(ProblemChanged);

    let input=commands.spawn((Text::new(""),
        InputText,
        DespawnOnExit(GameState::InGame),
        TextFont{
            font:japanese_font.0.clone().into(),
            font_size:FontSize::Px(50.0),
            ..default()
        })).id();    
    commands.entity(parent.0).add_child(input);
    let correct=asset_server.load("sounds/correct.mp3");
    let miss=asset_server.load("sounds/miss.mp3");
    commands.insert_resource(SoundAssets{correct,miss});
}

fn change_ui(
    typingstate:Res<TypingState>,
    mut target:Query<(&mut TextColor,&KanaSpan),Without<InputText>>,
    mut input:Query<&mut Text,(With<InputText>,Without<KanaSpan>)>){
        target.iter_mut().for_each(|(mut color,span)|{
            if span.0<typingstate.kana_index{
                color.0=Color::srgb(0.5,0.5,0.5);
            }
        });
        if let Ok(mut text)=input.single_mut(){
            *text=Text::new(typingstate.inputbuf.clone());
        }
    }

fn change_problem(
    mut msgs:MessageReader<ProblemChanged>,
    mut commands:Commands,
    target:ResMut<TargetTextEntity>,
    mut typingstate:ResMut<TypingState>,
    japanese_font:Res<JapaneseFont>
){

    for _ in msgs.read(){
        commands.entity(target.0).despawn_children();
        typingstate.inputbuf=String::new();
        for (i,s) in typingstate.odai_kana[typingstate.problem_index].iter().enumerate(){
            let child=commands.spawn((
                TextSpan::new(s.clone()),
                DespawnOnExit(GameState::InGame),
                 TextFont{
                        font:japanese_font.0.clone().into(),
                       font_size:FontSize::Px(50.0),
                        ..default()
                },
                TextColor(Color::WHITE),
                KanaSpan(i),
            )

            ).id();
            commands.entity(target.0).add_child(child);
        }
    }
}

fn handle_key(
    sounds:Res<SoundAssets>,
    mut commands:Commands,
    mut msg_kbd: MessageReader<KeyboardInput>,
    mut typingstate:ResMut<TypingState>,
    mut nok:ResMut<NOk>,
    mut msg_problem_changed: MessageWriter<ProblemChanged>,
    mut next_state:ResMut<NextState<GameState>>
){
    for msg in msg_kbd.read(){
        if msg.state==ButtonState::Released{continue;}
        if msg.repeat{continue;}
        if let Key::Character(input)=&msg.logical_key{
            for c in input.chars(){
                match typingstate.advance(c,&mut nok){
                    Ok(())=>{
                        commands.spawn((
                        AudioPlayer::new(sounds.correct.clone()),
                        PlaybackSettings::DESPAWN,));
                        typingstate.inputbuf.push(c);
                        println!("correct");
                    },
                    Err(())=>{
                        commands.spawn((
                            AudioPlayer::new(sounds.miss.clone()),
                        PlaybackSettings::DESPAWN));
                        println!("wrong");
                    }

                }
                if typingstate.kana_index==typingstate.product[typingstate.problem_index].len(){
                    if typingstate.problem_index+1==typingstate.problem_count{
                    println!("clear!");
                    next_state.set(GameState::EndMenu);
                    }
                    else {
                        typingstate.goto_next_problem();
                        msg_problem_changed.write(ProblemChanged);
                    }
                }
                println!("{:?}",typingstate);
            }
        }

}
}

fn start_endmenu(
    mut commands:Commands,
    parent:Res<TextParentEntity>
){
    let clear=commands.spawn((
        Text::new("CLEAR!"),
        DespawnOnExit(GameState::EndMenu),
    )).id();
    commands.entity(parent.0).add_child(clear);
}
fn return_to_mainmenu(
    mut next_state:ResMut<NextState<GameState>>,
    mut msg_kbd:MessageReader<KeyboardInput>,
    mut exit:MessageWriter<AppExit>,
    mut typingstate:ResMut<TypingState>
){
    for msg in msg_kbd.read(){
        match msg.logical_key{
            Key::Space=>{
                typingstate.initialize();
                next_state.set(GameState::MainMenu);
            },
            Key::Enter=>{
                exit.write(AppExit::Success);
            },
            _=>{continue;}
        }
    }
}

