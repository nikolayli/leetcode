// Time complexity: O(n^2)
// Space complexity: O(n)
impl Solution {
    pub fn minimum_beautiful_substrings(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![n + 1; n + 1];
        dp[0] = 0;
        let s = s.as_bytes();

        for i in 1..=n {
            if s[i - 1] == b'0' {
                continue;
            }

            let mut num = 0;
            for j in i..=n {
                num = (num << 1) + (s[j - 1] - b'0') as i32;
                if Self::is_power_of_five(num) {
                    dp[j] = dp[j].min(dp[i - 1] + 1);
                }
            }
        }

        if dp[n] == n + 1 {
            -1
        } else {
            dp[n] as i32
        }
    }

    fn is_power_of_five(mut num: i32) -> bool {
        while num % 5 == 0 && num > 0 {
            num /= 5;
        }

        num == 1
    }
}
