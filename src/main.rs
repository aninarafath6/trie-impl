use std::{collections::HashMap};

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

    fn insert(&mut self, word: &str) {
        let mut current_node = self;
        for c in word.chars() {
            current_node = current_node.children.entry(c).or_insert(TrieNode::new());
        }
        current_node.is_terminal_node = true
    }

    fn search(&self, word: &str) -> bool {
        let mut current_node = self;
        for c in word.chars() {
            if let Some(node) = current_node.children.get(&c) {
                current_node = node
            } else {
                return false;
            }
        }
        return current_node.is_terminal_node;
    }

    fn prefix(&self, prefix: &str) -> Vec<String> {
        let mut current = self;

        for c in prefix.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return Vec::new(),
            }
        }

        let mut results = Vec::new();
        let mut current_word = prefix.to_string();


        Self::dfs(&current,&mut current_word,&mut results);

        return results;
    }

    fn dfs(node:&TrieNode,current_word:&mut String,results:&mut Vec<String>){

        if node.is_terminal_node{
            results.push(current_word.clone());
        }

        for (ch,child_node) in &(node.children){
            current_word.push(*ch);
            Self::dfs(child_node, current_word, results);
            current_word.pop();
        }
    }
}

fn main() {
    let sample_text = "test";
    let mut trie = TrieNode::new();
    trie.insert(sample_text);
    trie.insert("anin");
    trie.insert("animal");
    trie.insert("animation");
    print!("have {:?}\n", trie.prefix("ani"));
}
