#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut k = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[k] = nums[i];
                k += 1;
            }
        }

        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! remove_duplicates_test {
        ($name:ident: $input:expr => ($expected_k:expr, $expected_nums:expr)) => {
            #[test]
            fn $name() {
                let mut nums = Vec::from($input);
                let k = Solution::remove_duplicates(&mut nums);
                assert_eq!(k, $expected_k);
                assert_eq!(&nums[..$expected_k as usize], $expected_nums);
            }
        };
    }

    remove_duplicates_test!(case1: [1, 1, 2] => (2, [1, 2]));
    remove_duplicates_test!(case2: [0, 0, 1, 1, 1, 2, 2, 3, 3, 4] => (5, [0, 1, 2, 3, 4]));
}
