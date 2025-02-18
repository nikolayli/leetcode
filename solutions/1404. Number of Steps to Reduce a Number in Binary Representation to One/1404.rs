// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut ans = 0;
        let mut chars: Vec<char> = s.chars().collect();

        while let Some(&'0') = chars.last() {
            chars.pop();
            ans += 1;
        }

        if chars.iter().collect::<String>() == "1" {
            return ans;
        }

        ans += 1;

        for &c in &chars {
            ans += if c == '1' { 1 } else { 2 };
        }

        ans
    }
}
