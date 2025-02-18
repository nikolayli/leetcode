// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut ans = Vec::new();
        let mut start = 0;

        for i in 0..nums.len() {
            if i > 0 && nums[i] != nums[i - 1] + 1 {
                start = i;
            }
            if i >= k - 1 {
                if i - start + 1 >= k {
                    ans.push(nums[i]);
                } else {
                    ans.push(-1);
                }
            }
        }

        ans
    }
}
