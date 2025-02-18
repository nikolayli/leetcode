// Time complexity: O(dw)
// Space complexity: O(d)
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, Rc<RefCell<TrieNode>>>,
    word: Option<String>,
}

impl Solution {
    pub fn replace_words(dictionery: Vec<String>, sentence: String) -> String {
        let root = Rc::new(RefCell::new(TrieNode::default()));
        for word in &dictionery {
            Self::insert(&root, word);
        }

        let mut ans = Vec::new();
        for s in sentence.split_whitespace() {
            ans.push(Self::search(&root, s).unwrap_or_else(|| s.to_string()));
        }

        ans.join(" ")
    }

    fn insert(root: &Rc<RefCell<TrieNode>>, word: &str) {
        let mut node = root.clone();
        for c in word.chars() {
            let child = {
                let mut node_borrow = node.borrow_mut();
                node_borrow
                    .children
                    .entry(c)
                    .or_insert_with(|| Rc::new(RefCell::new(TrieNode::default())))
                    .clone()
            };
            node = child;
        }
        node.borrow_mut().word = Some(word.to_string());
    }

    fn search(root: &Rc<RefCell<TrieNode>>, word: &str) -> Option<String> {
        let mut node = root.clone();
        for c in word.chars() {
            let temp_node = {
                let node_borrow = node.borrow();
                if let Some(w) = &node_borrow.word {
                    return Some(w.clone());
                }
                node_borrow.children.get(&c)?.clone()
            };
            node = temp_node;
        }
        let word_clone = node.borrow().word.clone();

        word_clone
    }
}
