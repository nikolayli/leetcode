// Time complexity: O(n)
// Space complexity: O(1)
use std::cmp;

impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut ans = 0;

        while l < r {
            if Self::is_prime(nums[l]) && Self::is_prime(nums[r]) {
                ans = ((r - l) as i32).abs();
                break;
            }

            if !Self::is_prime(nums[l]) {
                l += 1;
            }
            if !Self::is_prime(nums[r]) {
                r -= 1;
            }
        }

        ans
    }

    fn is_prime(num: i32) -> bool {
        if num <= 1 {
            return false;
        }

        for i in 2..=(num as f64).sqrt() as i32 {
            if num % i == 0 {
                return false;
            }
        }

        true
    }
}
