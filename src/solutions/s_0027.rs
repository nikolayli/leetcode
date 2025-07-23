struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! remove_element_test {
        ($name:ident: $nums:expr, $val:expr => $expected_k:expr, $expected_nums:expr) => {
            #[test]
            fn $name() {
                let mut nums = $nums.to_vec();
                let k = Solution::remove_element(&mut nums, $val);
                assert_eq!(k, $expected_k);

                let mut actual = nums[..k as usize].to_vec();
                actual.sort_unstable();
                let mut expected = $expected_nums.to_vec();
                expected.sort_unstable();

                assert_eq!(actual, expected);
            }
        };
    }

    remove_element_test!(case1: [3, 2, 2, 3], 3 => 2, [2, 2]);
    remove_element_test!(case2: [0, 1, 2, 2, 3, 0, 4, 2], 2 => 5, [0, 1, 4, 0, 3]);
}
