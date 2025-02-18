// Time complexity: O(n^2)
// Space complexity: O(1)
impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = i32::MAX;

        for i in 0..nums.len() {
            let mut curr = nums[i];
            if curr >= k {
                return 1;
            }

            for j in (i + 1)..nums.len() {
                curr |= nums[j];
                if curr >= k {
                    ans = ans.min((j - i + 1) as i32);
                    break;
                }
            }
        }

        return if ans == i32::MAX { -1 } else { ans };
    }
}
