// Time complexity: O(n*2^n)
// Space complexity: O(n^2*2^n)
use std::collections::HashMap;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        Self::ways(&expression, &mut HashMap::new())
    }

    fn ways(expression: &str, mem: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
        if let Some(result) = mem.get(expression) {
            return result.clone();
        }

        let mut ans = Vec::new();

        for (i, c) in expression.chars().enumerate() {
            if c == '+' || c == '-' || c == '*' {
                let left = Self::ways(&expression[..i], mem);
                let right = Self::ways(&expression[i + 1..], mem);

                for &a in &left {
                    for &b in &right {
                        let res = match c {
                            '+' => a + b,
                            '-' => a - b,
                            '*' => a * b,
                            _ => unreachable!(),
                        };
                        ans.push(res);
                    }
                }
            }
        }

        if ans.is_empty() {
            ans.push(expression.parse().unwrap());
        }

        mem.insert(expression.to_string(), ans.clone());
        ans
    }
}
