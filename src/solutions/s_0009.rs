#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut original = x;
        let mut reversed = 0;

        while original > 0 {
            reversed = reversed * 10 + original % 10;
            original /= 10;
        }

        x == reversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! is_palindrome_test {
        ($name:ident: $x:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::is_palindrome($x), $expected);
            }
        };
    }

    is_palindrome_test!(case1: 121 => true);
    is_palindrome_test!(case2: -121 => false);
    is_palindrome_test!(case3: 10 => false);
}

