use std::{collections::HashMap, fs};

pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_terminal_node: bool,
}

impl TrieNode {
    pub fn new() -> TrieNode {
        Self {
            children: HashMap::new(),
            is_terminal_node: false,
        }
    }

    pub fn new_with_data() -> TrieNode {
        let mut trie: TrieNode = TrieNode::new();

        let words =
            fs::read_to_string("./assets/english_words.txt").expect("Unable to read words!!");

        for word in words.lines() {
            trie.insert(word);
        }

        return trie;
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = self;
        for c in word.chars() {
            current_node = current_node.children.entry(c).or_insert(TrieNode::new());
        }
        current_node.is_terminal_node = true
    }

    pub fn search(&self, word: &str) -> bool {
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

    pub fn prefix(&self, prefix: &str) -> Vec<String> {
        let mut current = self;

        for c in prefix.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return Vec::new(),
            }
        }

        let mut results = Vec::new();
        let mut current_word = prefix.to_string();

        Self::dfs(&current, &mut current_word, &mut results);

        return results;
    }

    fn dfs(node: &TrieNode, current_word: &mut String, results: &mut Vec<String>) {
        if node.is_terminal_node {
            results.push(current_word.clone());
        }

        for (ch, child_node) in &(node.children) {
            current_word.push(*ch);
            Self::dfs(child_node, current_word, results);
            current_word.pop();
        }
    }
}
