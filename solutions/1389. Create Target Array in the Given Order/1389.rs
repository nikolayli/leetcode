// Time complexity: O(n^2)
// Space complexity: O(n)
impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();

        for (i, &num) in nums.iter().enumerate() {
            ans.insert(index[i] as usize, num);
        }

        ans
    }
}
