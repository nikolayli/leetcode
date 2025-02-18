// Time complexity: O(n^2)
// Space complexity: O(n)
impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        let mut ans = vec![];

        fn dfs(n: i32, s: &mut Vec<char>, ans: &mut Vec<String>) {
            if n == 0 {
                ans.push(s.iter().collect());
                return;
            }
            if s.is_empty() || *s.last().unwrap() == '1' {
                s.push('0');
                dfs(n - 1, s, ans);
                s.pop();
            }
            s.push('1');
            dfs(n - 1, s, ans);
            s.pop();
        }

        dfs(n, &mut vec![], &mut ans);
        ans
    }
}
