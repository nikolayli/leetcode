struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return String::new();
        }

        let bytes = s.as_bytes();

        let (mut start, mut end) = (0, 0);
        for i in 0..bytes.len() {
            let len1 = Self::expand_around_center(bytes, i, i);
            let len2 = Self::expand_around_center(bytes, i, i + 1);
            let len = len1.max(len2);

            if len > end - start {
                start = i - (len - 1) / 2;
                end = i + len / 2;
            }
        }

        s[start..=end].to_string()
    }

    fn expand_around_center(s: &[u8], left: usize, right: usize) -> usize {
        let (mut l, mut r) = (left as isize, right as isize);

        while l >= 0 && r < s.len() as isize && s[l as usize] == s[r as usize] {
            l -= 1;
            r += 1;
        }

        (r - l - 1) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! longest_palindrome_test {
        ($name:ident: $s:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::longest_palindrome($s.to_string()),
                    $expected.to_string()
                );
            }
        };
    }

    longest_palindrome_test!(case1: "babad" => "aba");
    longest_palindrome_test!(case2: "cbbd" => "bb");
}
