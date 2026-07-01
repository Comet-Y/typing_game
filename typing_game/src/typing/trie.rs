#[derive(Clone, Debug, Default)]
pub struct TrieTree {
    pub trie: Vec<TrieNode>,
}

impl TrieTree {
    pub fn new() -> Self {
        Self {
            trie: vec![TrieNode {
                is_end_node: false,
                ..Default::default()
            }],
        }
    }

    pub fn insert(&mut self, adds: &[&str]) {
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

#[derive(Default, Clone, Debug)]
pub struct TrieNode {
    pub is_end_node: bool,
    pub is_n: bool,
    pub next_nodes: std::collections::HashMap<char, usize>,
}
