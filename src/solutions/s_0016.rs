use std::cmp::Ordering;

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort_unstable();

        let mut ans = nums[0] + nums[1] + nums[2];

        for i in 0..n - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = n - 1;

            while l < r {
                let curr = nums[i] + nums[l] + nums[r];

                if (curr - target).abs() < (ans - target).abs() {
                    ans = curr;
                }

                match curr.cmp(&target) {
                    Ordering::Less => {
                        l += 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                    }
                    Ordering::Greater => {
                        r -= 1;
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    }
                    Ordering::Equal => {
                        return curr;
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

    macro_rules! three_sum_closest_test {
        ($name:ident: $nums:expr, $target:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::three_sum_closest(Vec::from($nums), $target),
                    $expected
                );
            }
        };
    }

    three_sum_closest_test!(case1: [-1, 2, 1, -4], 1 => 2);
    three_sum_closest_test!(case2: [0, 0, 0], 1 => 0);
}
