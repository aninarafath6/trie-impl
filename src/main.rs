mod trie;

use crate::trie::TrieNode;

fn main() {
    let trie: TrieNode = TrieNode::new_with_data();

    println!("{:?}\n",trie.prefix("car"));
    // ["car", "card", "carbon", "carrier", "carry", "care", "careful", "carefully", "career"]
}
