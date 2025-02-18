// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = Vec::new();

        for c in s.chars() {
            match c {
                'B' => {
                    if let Some('A') = stack.last() {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                }
                'D' => {
                    if let Some('C') = stack.last() {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                }
                _ => stack.push(c),
            }
        }

        stack.len() as i32
    }
}
