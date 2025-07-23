struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();
        let n = s_chars.len();
        let m = p_chars.len();

        let mut dp = vec![vec![false; m + 1]; n + 1];
        dp[0][0] = true;

        for j in 1..=m {
            if p_chars[j - 1] == '*' {
                dp[0][j] = dp[0][j - 2];
            }
        }

        for i in 1..=n {
            for j in 1..=m {
                let sc = s_chars[i - 1];
                let pc = p_chars[j - 1];

                if pc != '*' {
                    dp[i][j] = dp[i - 1][j - 1] && (sc == pc || pc == '.');
                } else {
                    dp[i][j] = dp[i][j - 2];
                    if j >= 2 {
                        let prev_pc = p_chars[j - 2];
                        if prev_pc == '.' || prev_pc == sc {
                            dp[i][j] = dp[i][j] || dp[i - 1][j];
                        }
                    }
                }
            }
        }

        dp[n][m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! is_match_test {
        ($name:ident: $s:expr, $p:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::is_match($s.to_string(), $p.to_string()),
                    $expected
                );
            }
        };
    }

    is_match_test!(case1: "aa", "a" => false);
    is_match_test!(case2: "aa", "a*" => true);
    is_match_test!(case3: "ab", ".*" => true);
}
