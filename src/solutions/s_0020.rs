#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => if stack.pop() != Some('(') { return false },
                ']' => if stack.pop() != Some('[') { return false },
                '}' => if stack.pop() != Some('{') { return false },
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! is_valid_test {
        ($name:ident: $s:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::is_valid($s.to_string()), $expected);
            }
        };
    }

    is_valid_test!(case1: "()" => true);
    is_valid_test!(case2: "()[]{}" => true);
    is_valid_test!(case3: "(]" => false);
    is_valid_test!(case4: "([])" => true);
}
