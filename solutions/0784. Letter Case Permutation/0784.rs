// # Time complexity: O(2^n)
// Space complexity: O(2^n)
impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        fn backtrack(s: &mut Vec<char>, i: usize, ans: &mut Vec<String>) {
            if i == s.len() {
                ans.push(s.iter().collect());
                return;
            }

            if s[i].is_alphabetic() {
                s[i] = s[i].to_ascii_lowercase();
                backtrack(s, i + 1, ans);
                s[i] = s[i].to_ascii_uppercase();
                backtrack(s, i + 1, ans);
            } else {
                backtrack(s, i + 1, ans);
            }
        }

        let mut ans = Vec::new();
        let mut chars: Vec<char> = s.chars().collect();
        backtrack(&mut chars, 0, &mut ans);

        ans
    }
}
