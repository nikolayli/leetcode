struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        let mut curr = String::with_capacity((2 * n) as usize);
        Self::dfs(n, 0, 0, &mut curr, &mut ans);
        ans
    }

    fn dfs(n: i32, open: i32, close: i32, curr: &mut String, ans: &mut Vec<String>) {
        if open == n && close == n {
            ans.push(curr.clone());
            return;
        }

        if open < n {
            curr.push('(');
            Self::dfs(n, open + 1, close, curr, ans);
            curr.pop();
        }

        if close < open {
            curr.push(')');
            Self::dfs(n, open, close + 1, curr, ans);
            curr.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_parenthesis_test {
        ($name:ident: $n:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let mut ans = Solution::generate_parenthesis($n);
                let mut expected: Vec<String> = $expected.iter().map(|&s| s.to_string()).collect();

                ans.sort_unstable();
                expected.sort_unstable();

                assert_eq!(ans, expected);
            }
        };
    }

    generate_parenthesis_test!(case1: 3 => ["((()))", "(()())", "(())()", "()(())", "()()()"]);
    generate_parenthesis_test!(case2: 1 => ["()"]);
}
