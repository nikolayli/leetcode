// Time complexity : O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut ans = Vec::new();
        let mut pair = HashMap::new();
        let mut stack = Vec::new();

        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => stack.push(i),
                ')' => {
                    if let Some(j) = stack.pop() {
                        pair.insert(i, j);
                        pair.insert(j, i);
                    }
                }
                _ => {}
            }
        }

        let (mut i, mut d) = (0, 1);
        while i < s.len() {
            match s.chars().nth(i) {
                Some(c) if c == '(' || c == ')' => {
                    i = *pair.get(&i).unwrap();
                    d = -d;
                }
                Some(c) => ans.push(c),
                None => break,
            }
            i = (i as isize + d) as usize;
        }

        ans.into_iter().collect()
    }
}
