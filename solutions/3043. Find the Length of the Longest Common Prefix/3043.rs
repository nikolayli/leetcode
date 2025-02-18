// Time complexity: O(m+n)
// Space complexity: O(m)
use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
        }
    }
}

struct Trie {
    root: Box<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: Box::new(TrieNode::new()),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut *self.root;

        for c in word.chars() {
            node = node
                .children
                .entry(c)
                .or_insert_with(|| Box::new(TrieNode::new()));
        }
    }

    fn search(&self, word: &str) -> usize {
        let mut prefix_length = 0;
        let mut node = &*self.root;

        for c in word.chars() {
            if let Some(next_node) = node.children.get(&c) {
                node = next_node;
                prefix_length += 1;
            } else {
                break;
            }
        }

        prefix_length
    }
}

impl Solution {
    fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        let mut ans = 0;

        for num in arr1 {
            trie.insert(&num.to_string());
        }

        for num in arr2 {
            ans = ans.max(trie.search(&num.to_string()) as i32);
        }

        ans
    }
}
