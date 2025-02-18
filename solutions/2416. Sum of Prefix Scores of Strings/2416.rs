// Time complexity: O(mn)
// Space complexity: O(mn)
struct TrieNode {
    next: [Option<Box<TrieNode>>; 26],
    count: usize,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            next: Default::default(),
            count: 0,
        }
    }
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut root = TrieNode::new();
        let n = words.len();
        let mut ans = vec![0; n];

        for word in &words {
            Self::insert(&mut root, word);
        }

        for (i, word) in words.iter().enumerate() {
            ans[i] = Self::count(&root, word);
        }

        ans
    }

    fn insert(root: &mut TrieNode, word: &str) {
        let mut node = root;

        for c in word.chars() {
            let index = (c as usize) - ('a' as usize);

            if node.next[index].is_none() {
                node.next[index] = Some(Box::new(TrieNode::new()));
            }
            node.next[index].as_mut().unwrap().count += 1;
            node = node.next[index].as_mut().unwrap();
        }
    }

    fn count(root: &TrieNode, s: &str) -> i32 {
        let mut node = root;
        let mut res = 0;

        for c in s.chars() {
            let index = (c as usize) - ('a' as usize);

            if let Some(ref next_node) = node.next[index] {
                res += next_node.count;
                node = next_node;
            } else {
                break;
            }
        }

        res as i32
    }
}
