use std::collections::HashMap;

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_to_index = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = num_to_index.get(&complement) {
                return vec![index, i as i32];
            }
            num_to_index.insert(num, i as i32);
        }

        unreachable!("No two sum solution")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! two_sum_test {
        ($name:ident: $nums:expr, $target:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::two_sum($nums.into(), $target), $expected);
            }
        };
    }

    two_sum_test!(test1: [2, 7, 11, 15], 9 => vec![0, 1]);
    two_sum_test!(test2: [2, 3, 4], 6 => vec![0, 2]);
    two_sum_test!(test3: [3, 3], 6 => vec![0, 1]);
}
