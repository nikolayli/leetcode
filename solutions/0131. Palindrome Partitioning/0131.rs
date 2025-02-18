// Time complexity: O(n*2^n)
// Space complexity: O(n*2^n)
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut ans: Vec<Vec<String>> = Vec::new();

        fn is_palindrome(s: &str) -> bool {
            s.chars().eq(s.chars().rev())
        }

        fn dfs(s: &str, j: usize, path: Vec<String>, ans: &mut Vec<Vec<String>>) {
            if j == s.len() {
                ans.push(path);
                return;
            }

            for i in j..s.len() {
                if is_palindrome(&s[j..=i]) {
                    let mut new_path = path.clone();
                    new_path.push(s[j..=i].to_string());
                    dfs(s, i + 1, new_path, ans);
                }
            }
        }

        dfs(&s, 0, Vec::new(), &mut ans);
        ans
    }
}
