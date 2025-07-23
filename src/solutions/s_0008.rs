struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut iter = s.chars().peekable();

        while let Some(' ') = iter.peek() {
            iter.next();
        }

        let sign = match iter.peek() {
            Some('-') => {
                iter.next();
                -1
            }
            Some('+') => {
                iter.next();
                1
            }
            _ => 1,
        };

        let mut num: i32 = 0;
        let mut overflow = false;

        while let Some(&c) = iter.peek() {
            if let Some(digit) = c.to_digit(10) {
                iter.next();
                let digit = digit as i32;

                let next_val = if sign == 1 {
                    num.checked_mul(10).and_then(|n| n.checked_add(digit))
                } else {
                    num.checked_mul(10).and_then(|n| n.checked_sub(digit))
                };

                match next_val {
                    Some(n) => num = n,
                    None => {
                        overflow = true;
                        break;
                    }
                }
            } else {
                break;
            }
        }

        if overflow {
            if sign == 1 {
                i32::MAX
            } else {
                i32::MIN
            }
        } else {
            num
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! my_atoi_test {
        ($name:ident: $s:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::my_atoi($s.to_string()), $expected);
            }
        };
    }

    my_atoi_test!(case1: "42" => 42);
    my_atoi_test!(case2: "   -042" => -42);
    my_atoi_test!(case3: "1337c0d3" => 1337);
    my_atoi_test!(case4: "0-1" => 0);
    my_atoi_test!(case5: "words and 987" => 0);
}
