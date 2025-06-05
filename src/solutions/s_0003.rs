#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ans = 0;
        let mut count = vec![0; 128];
        let mut l = 0;
        let bytes = s.as_bytes();

        for r in 0..bytes.len() {
            count[bytes[r] as usize] += 1;

            while count[bytes[r] as usize] > 1 {
                count[bytes[l] as usize] -= 1;
                l += 1;
            }

            ans = std::cmp::max(ans, r - l + 1);
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! length_of_longest_substring_test {
        ($name:ident: $input:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::length_of_longest_substring($input.to_string()),
                    $expected
                );
            }
        };
    }

    length_of_longest_substring_test!(case1: "abcabcbb" => 3);
    length_of_longest_substring_test!(case2: "bbbbb" => 1);
    length_of_longest_substring_test!(case3: "pwwkew" => 3);
}
