#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let values = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut ans = String::new();

        for &(value, symbol) in values.iter() {
            while num >= value {
                ans.push_str(symbol);
                num -= value;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! int_to_roman_test {
        ($name:ident: $num:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::int_to_roman($num), $expected);
            }
        };
    }

    int_to_roman_test!(case1: 3749 => "MMMDCCXLIX");
    int_to_roman_test!(case2: 58 => "LVIII");
    int_to_roman_test!(case3: 1994 => "MCMXCIV");
}
