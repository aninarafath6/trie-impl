use std::collections::HashMap;

trait Trie {
    fn insert(&self, string:&String);
    fn serach(&self, string:&String);
}
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_terminal_node: bool,
}

impl TrieNode {
    fn new() -> TrieNode {
        Self {
            children: HashMap::new(),
            is_terminal_node: false,
        }
    }

    fn insert(&mut self, new_str: &str) {
        let mut current_node = self;
        for c in new_str.chars() {
            if !current_node.children.contains_key(&c) {
                current_node.children.insert(c, TrieNode::new());
            }
            current_node = current_node.children.get_mut(&c).unwrap();
        }
        current_node.is_terminal_node = true
    }
}

fn main() {
    let sample_text = "anin";

    let mut trie = TrieNode::new();

    trie.insert(sample_text);
}
