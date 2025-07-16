#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n < 3 {
            return vec![];
        }

        nums.sort_unstable();
        let mut ans = Vec::new();

        for i in 0..n - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                let total = nums[i] + nums[l] + nums[r];

                match total.cmp(&0) {
                    std::cmp::Ordering::Equal => {
                        ans.push(vec![nums[i], nums[l], nums[r]]);

                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        l += 1;
                        r -= 1;
                    }
                    std::cmp::Ordering::Less => {
                        l += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        r -= 1;
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    macro_rules! three_sum_test {
        ($name:ident: $nums:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let nums = $nums;
                let mut result = Solution::three_sum(nums.to_vec());
                let mut expected: Vec<Vec<i32>> = $expected;

                for triple in result.iter_mut() {
                    triple.sort_unstable();
                }
                for triple in expected.iter_mut() {
                    triple.sort_unstable();
                }

                let result_set: HashSet<Vec<i32>> = result.into_iter().collect();
                let expected_set: HashSet<Vec<i32>> = expected.into_iter().collect();

                assert_eq!(result_set, expected_set);
            }
        };
    }

    three_sum_test!(case1: [-1, 0, 1, 2, -1, -4] => vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    three_sum_test!(case2: [0, 1, 1] => vec![]);
    three_sum_test!(case3: [0, 0, 0] => vec![vec![0, 0, 0]]);
}
