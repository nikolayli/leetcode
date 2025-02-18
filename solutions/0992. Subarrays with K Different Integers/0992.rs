// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        Self::subarray_with_at_most_k_distinct(&nums, k)
            - Self::subarray_with_at_most_k_distinct(&nums, k - 1)
    }

    fn subarray_with_at_most_k_distinct(nums: &Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut count = vec![0; nums.len() + 1];

        let (mut l, mut r, mut distinct) = (0, 0, 0);
        while r < nums.len() {
            if count[nums[r] as usize] == 0 {
                distinct += 1;
            }
            count[nums[r] as usize] += 1;

            while distinct > k {
                count[nums[l] as usize] -= 1;
                if count[nums[l] as usize] == 0 {
                    distinct -= 1;
                }
                l += 1;
            }

            ans += r - l + 1;
            r += 1;
        }

        ans as i32
    }
}
