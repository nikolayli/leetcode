#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n < 4 {
            return vec![];
        }

        let mut nums = nums;
        nums.sort_unstable();
        let target = target as i64;
        let mut ans = Vec::new();

        for i in 0..n - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            for j in i + 1..n - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                let mut l = j + 1;
                let mut r = n - 1;
                let two_sum = (nums[i] + nums[j]) as i64;

                while l < r {
                    let four_sum = two_sum + nums[l] as i64 + nums[r] as i64;

                    if four_sum == target {
                        ans.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                        l += 1;
                        r -= 1;

                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    } else if four_sum < target {
                        l += 1;
                    } else {
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

    macro_rules! four_sum_test {
        ($name:ident: $nums:expr, $target:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let mut ans = Solution::four_sum($nums.into(), $target);
                let mut expected: Vec<Vec<i32>> =
                    $expected.into_iter().map(|v| v.to_vec()).collect();

                for vec in ans.iter_mut() {
                    vec.sort_unstable();
                }
                for vec in expected.iter_mut() {
                    vec.sort_unstable();
                }
                ans.sort_unstable();
                expected.sort_unstable();

                assert_eq!(ans, expected);
            }
        };
    }

    four_sum_test!(case1: vec![1, 0, -1, 0, -2, 2], 0 => vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]);
    four_sum_test!(case2: vec![2, 2, 2, 2, 2], 8 => vec![vec![2, 2, 2, 2]]);
}
