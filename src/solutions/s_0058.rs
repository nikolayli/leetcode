#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().unwrap().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! length_of_last_word_test {
        ($name:ident: $input:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::length_of_last_word($input.to_string()), $expected);
            }
        };
    }

    length_of_last_word_test!(case1: "Hello World" => 5);
    length_of_last_word_test!(case2: "   fly me   to   the moon  " => 4);
    length_of_last_word_test!(case3: "luffy is still joyboy" => 6);
}
