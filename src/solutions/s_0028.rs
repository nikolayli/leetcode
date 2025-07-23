struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map_or(-1, |idx| idx as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! str_str_test {
        ($name:ident: $haystack:expr, $needle:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    str_str($haystack.to_string(), $needle.to_string()),
                    $expected
                );
            }
        };
    }

    str_str_test!(case1: "sadbutsad", "sad" => 0);
    str_str_test!(case2: "leetcode", "leeto" => -1);
}
