// Time complexity: O(nlog(S))
// Space complexity: O(1)
impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        const k_mod: i64 = 1_000_000_007;

        let subarrays_and_sum_no_greater_than = |nums: &Vec<i32>, m: i32| -> (i32, i64) {
            let mut count = 0;
            let mut total = 0;
            let mut sum = 0;
            let mut window = 0;
            let mut i = 0;

            for j in 0..nums.len() {
                sum += nums[j] * (j - i + 1) as i32;
                window += nums[j];

                while window > m {
                    sum -= window;
                    window -= nums[i];
                    i += 1;
                }

                count += (j - i + 1) as i32;
                total += sum as i64;
            }

            (count, total)
        };

        let l = *nums.iter().min().unwrap();
        let r = nums.iter().sum::<i32>();

        let first_k_subarrays_sum = |nums: &Vec<i32>, k: i32| -> i64 {
            let mut l = l;
            let mut r = r;

            while l < r {
                let m = l + (r - l) / 2;
                if subarrays_and_sum_no_greater_than(nums, m).0 < k {
                    l = m + 1;
                } else {
                    r = m;
                }
            }

            let (count, total) = subarrays_and_sum_no_greater_than(nums, l);
            total - l as i64 * (count - k) as i64
        };

        let left_sum = first_k_subarrays_sum(&nums, left - 1);
        let right_sum = first_k_subarrays_sum(&nums, right);

        ((right_sum - left_sum + k_mod) % k_mod) as i32
    }
}
