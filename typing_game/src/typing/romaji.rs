use crate::typing::trie;
use crate::typing::session;
use bevy::prelude::*;
#[derive(Debug,Clone,Copy)]
pub struct TrieId(usize);
#[derive(Resource)]
pub struct RomajiDictionary{
    tries:Vec<trie::TrieTree>,
    kana_to_trie_map:std::collections::HashMap<String,TrieId>
}

impl RomajiDictionary{
    pub fn new()->Self{
        let (tries,kana_to_trie_map)=build_tries();
        Self {
            tries,
            kana_to_trie_map,
        }
    }
    pub fn build_kana_units(&self,odai_kana:&str)->Vec<session::KanaUnit>{
        build_kana_units(odai_kana,&self.kana_to_trie_map)
    }
    pub fn get_trie(&self,trie_id:TrieId)->&trie::TrieTree{
        &self.tries[trie_id.0]
    }
}
fn build_tries() -> (Vec<trie::TrieTree>, std::collections::HashMap<String, TrieId>) {
    let mut tries = Vec::new();
    let mut kana_to_trie_map = std::collections::HashMap::new();
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
        let mut t = trie::TrieTree::new();
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
        kana_to_trie_map.insert(s.to_string(), TrieId(i));
    });

    (tries, kana_to_trie_map)
}

fn build_kana_units(
    input: &str,
    kana_to_trie_map: &std::collections::HashMap<String, TrieId>,
) -> Vec<session::KanaUnit> {
    let mut kana_units=Vec::new();
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
            if let Some(index) = kana_to_trie_map.get(&format! {"{}{}",one,two}) {
                skip = true;
                kana_units.push(session::KanaUnit::new(format!("{}{}",one,two),*index));
                return;
            }
                if let Some(index) = kana_to_trie_map.get(&one.to_string()) {
                kana_units.push(session::KanaUnit::new(one.to_string(),*index));
                return;
                } 
                if one == 'ん' {
                    let index=if "あいうえおなにぬねのやゆよ".contains(two) {
                        kana_to_trie_map.get("んa").unwrap()

                    } else {
                        kana_to_trie_map.get("んb").unwrap()
                    };
                    kana_units.push(session::KanaUnit::new("ん".to_string(),*index));
                    return;
                } else {
                    println!("{}{}", one, two);
                    panic!("ERROR:couldn't make trie tree");
                }
            
        });
    kana_units
}