struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut total = 0;

        for i in 0..n - 1 {
            let curr = Self::value_of(chars[i]);
            let next = Self::value_of(chars[i + 1]);
            if curr < next {
                total -= curr;
            } else {
                total += curr;
            }
        }
        total + Self::value_of(chars[n - 1])
    }

    fn value_of(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! roman_to_int_test {
        ($name:ident: $s:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::roman_to_int($s.to_string()), $expected);
            }
        };
    }

    roman_to_int_test!(case1: "III" => 3);
    roman_to_int_test!(case2: "LVIII" => 58);
    roman_to_int_test!(case3: "MCMXCIV" => 1994);
}
