struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut ans: i32 = 0;

        while x != 0 {
            let digit = x % 10;
            x /= 10;

            ans = match ans.checked_mul(10).and_then(|n| n.checked_add(digit)) {
                Some(n) => n,
                None => return 0,
            };
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! reverse_test {
        ($name:ident: $x:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::reverse($x), $expected);
            }
        };
    }

    reverse_test!(case1: 123 => 321);
    reverse_test!(case2: -123 => -321);
    reverse_test!(case3: 120 => 21);
}
