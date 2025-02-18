// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        fn number_of_subarrays_at_most(nums: &[i32], k: i32) -> i32 {
            nums.iter()
                .enumerate()
                .scan((0, k), |(l, k), (r, &num)| {
                    if num % 2 == 1 {
                        *k -= 1;
                    }

                    while *k < 0 {
                        if nums[*l] % 2 == 1 {
                            *k += 1;
                        }
                        *l += 1;
                    }
                    Some((r - *l) as i32)
                })
                .sum()
        }

        number_of_subarrays_at_most(&nums, k) - number_of_subarrays_at_most(&nums, k - 1)
    }
}
