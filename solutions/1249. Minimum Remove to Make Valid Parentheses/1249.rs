// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack: Vec<usize> = Vec::new();
        let mut modified: Vec<char> = s.chars().collect();

        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i);
            } else if c == ')' {
                if stack.is_empty() {
                    modified[i] = '*';
                } else {
                    stack.pop();
                }
            }
        }

        for i in stack {
            modified[i] = '*';
        }

        modified.retain(|&c| c != '*');

        modified.into_iter().collect()
    }
}
