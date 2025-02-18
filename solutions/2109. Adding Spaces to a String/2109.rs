// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut ans = String::with_capacity(s.len() + spaces.len());
        let mut j = 0;
        let s_chars: Vec<char> = s.chars().collect();

        for (i, &c) in s_chars.iter().enumerate() {
            if j < spaces.len() && i == spaces[j] as usize {
                ans.push(' ');
                j += 1;
            }
            ans.push(c);
        }

        ans
    }
}
