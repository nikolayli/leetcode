#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digit.is_empty() {
            return vec![];
        }

        let mapping = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

        let mut ans = vec![String::new()];

        for d in digits.chars() {
            let letters = mapping[(d as u8 - b'2') as usize];
            let mut temp = Vec::new();

            for s in &ans {
                for l in letters.chars() {
                    temp.push(format!("{}{}", s, l));
                }
            }
            ans = temp;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! letter_combinations_test {
        ($name:ident: $digits:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let mut ans = Solution::letter_combinations($digits);
                let mut expected: Vec<String> =
                    $expected.into_iter().map(|s| s.to_string()).collect();
                ans.sort_unstable();
                expected.sort_unstable();
                assert_eq!(ans, expected);
            }
        };
    }

    letter_combinations_test!(case1: "23" => vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
    letter_combinations_test!(case2: "" => vec![]);
    letter_combinations_test!(case3: "2" => vec!["a", "b", "c"]);
}
