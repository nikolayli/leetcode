#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        let first = strs[0].as_bytes();
        for (i, &ch) in first.iter().enumerate() {
            for s in strs.iter().skip(1) {
                let bytes = s.as_bytes();
                if i >= bytes.len() || bytes[i] != ch {
                    return String::from_utf8_lossy(&first[0..i]).to_string();
                }
            }
        }
        strs[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! longest_common_prefix_test {
        ($name:ident: $strs:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let strs: Vec<String> = $strs.into_iter().map(String::from).collect();
                assert_eq!(Solution::longest_common_prefix(strs), $expected);
            }
        };
    }

    longest_common_prefix_test!(case1: ["flower", "flow", "flight"] => "fl");
    longest_common_prefix_test!(case2: ["dog", "racecar", "car"] => "");
}
